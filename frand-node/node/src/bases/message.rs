use std::io::Cursor;
use std::fmt::Debug;
use result::MessageError;
use serde::{Deserialize, Serialize};
use crate::{*, result::{NodeError, Result}};

pub type PayloadId = u32;
pub type PayloadKey = Box<[PayloadId]>;
pub type PayloadDepth = u32;

pub trait MessageBase: Debug + Clone + Sized {
    fn from_payload(depth: usize, payload: Payload) -> Self;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Payload {
    key: PayloadKey,
    state: Box<[u8]>,
}

impl Payload {
    pub fn key(&self) -> &PayloadKey { &self.key }

    pub fn new<S: StateBase>(
        key: PayloadKey, 
        state: S,
    ) -> Self {
        Self { 
            key, 
            state: Self::serialize(state), 
        }
    }

    pub fn serialize<S: StateBase>(state: S) -> Box<[u8]>{
        let mut buffer = Vec::new();

        ciborium::into_writer(&state, &mut buffer)
        .unwrap_or_else(|err| 
            panic!("serialize {:#?} into CBOR -> Err({err})", state)
        );

        buffer.into_boxed_slice()
    }

    pub fn read_state<S: StateBase>(&self) -> S {
        ciborium::from_reader(Cursor::new(&self.state))
        .unwrap_or_else(|err| 
            panic!("deserialize CBOR with {:#?} -> Err({err})", self.state)
        )
    }

    pub fn get_id(&self, depth: usize) -> Option<PayloadId> { 
        self.key.get(depth).copied()
    }

    pub fn error(
        &self, 
        depth: usize,
        message: impl AsRef<str>,
    ) -> NodeError {
        MessageError { 
            depth: depth as PayloadDepth, 
            key: self.key.clone(), 
            state: self.state.clone(), 
            message: message.as_ref().to_owned(), 
        }.into()
    }
}

impl TryFrom<Payload> for Vec<u8> {    
    type Error = NodeError;

    fn try_from(payload: Payload) -> Result<Self> {        
        let mut buffer = Vec::new();

        match ciborium::into_writer(&payload, &mut buffer) {
            Ok(()) => Ok(buffer),
            Err(err) => Err(payload.error(0, err.to_string())),
        }
    }
}

impl TryFrom<Vec<u8>> for Payload {    
    type Error = NodeError;

    fn try_from(data: Vec<u8>) -> Result<Self> {      
        ciborium::from_reader(Cursor::new(data))
        .map_err(|err| NodeError::Text(err.to_string()))
    }
}

impl From<anyhow::Result<Vec<u8>>> for Payload {    
    fn from(data: anyhow::Result<Vec<u8>>) -> Self {      
        data.unwrap().try_into().unwrap()
    }
}

impl TryFrom<Payload> for String 
{    
    type Error = NodeError;

    fn try_from(payload: Payload) -> Result<Self> {       
        Ok(String::from_utf8(payload.try_into()?)?)
    }
}

impl TryFrom<String> for Payload {    
    type Error = NodeError;

    fn try_from(data: String) -> Result<Self> {      
        data.into_bytes().try_into()
    }
}

impl From<anyhow::Result<String>> for Payload {   
    fn from(data: anyhow::Result<String>) -> Self {      
        data.unwrap().try_into().unwrap()
    }
}
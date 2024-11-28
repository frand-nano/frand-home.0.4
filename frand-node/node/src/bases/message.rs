use std::io::Cursor;
use std::fmt::Debug;
use result::MessageError;
use serde::{Deserialize, Serialize};
use crate::{*, result::{NodeError, Result}};

pub type NodeId = u32;
pub type NodeKey = Box<[NodeId]>;
pub type NodeDepth = u32;

pub trait MessageBase: Debug + Clone + Sized {
    fn from_packet(depth: usize, packet: &Packet) -> Self;
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Packet {
    key: NodeKey,
    state: Box<[u8]>,
}

impl Packet {
    pub fn key(&self) -> &NodeKey { &self.key }

    pub fn new<S: StateBase>(
        key: NodeKey, 
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

    pub fn get_id(&self, depth: usize) -> Option<NodeId> { 
        self.key.get(depth).copied()
    }

    pub fn error(
        &self, 
        depth: usize,
        message: impl AsRef<str>,
    ) -> NodeError {
        MessageError { 
            depth: depth as NodeDepth, 
            key: self.key.clone(), 
            state: self.state.clone(), 
            message: message.as_ref().to_owned(), 
        }.into()
    }
}

impl TryFrom<&Packet> for Vec<u8> {    
    type Error = NodeError;

    fn try_from(packet: &Packet) -> Result<Self> {        
        let mut buffer = Vec::new();

        match ciborium::into_writer(packet, &mut buffer) {
            Ok(()) => Ok(buffer),
            Err(err) => Err(packet.error(0, err.to_string())),
        }
    }
}

impl TryFrom<Vec<u8>> for Packet {    
    type Error = NodeError;

    fn try_from(data: Vec<u8>) -> Result<Self> {      
        ciborium::from_reader(Cursor::new(data))
        .map_err(|err| NodeError::Text(err.to_string()))
    }
}

impl From<anyhow::Result<Vec<u8>>> for Packet {    
    fn from(data: anyhow::Result<Vec<u8>>) -> Self {      
        data.unwrap().try_into().unwrap()
    }
}

impl TryFrom<&Packet> for String 
{    
    type Error = NodeError;

    fn try_from(packet: &Packet) -> Result<Self> {       
        Ok(String::from_utf8(packet.try_into()?)?)
    }
}

impl TryFrom<String> for Packet {    
    type Error = NodeError;

    fn try_from(data: String) -> Result<Self> {      
        data.into_bytes().try_into()
    }
}

impl From<anyhow::Result<String>> for Packet {   
    fn from(data: anyhow::Result<String>) -> Self {      
        data.unwrap().try_into().unwrap()
    }
}
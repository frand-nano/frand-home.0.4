use std::io::Cursor;
use std::fmt::Debug;
use result::MessageError;
use serde::{Deserialize, Serialize};
use crate::{*, result::{NodeError, Result}};

pub type MessageDataId = u32;
pub type MessageDataKey = Box<[MessageDataId]>;
pub type MessageDataDepth = u32;

pub trait MessageBase: Debug + Clone + Sized {
    fn deserialize(depth: usize, data: MessageData) -> Self;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    key: MessageDataKey,
    value: Box<[u8]>,
}

impl MessageData {
    pub fn key(&self) -> &MessageDataKey { &self.key }

    pub fn new<S: StateBase>(
        key: &MessageDataKey, 
        id: Option<MessageDataId>, 
        value: S,
    ) -> Result<Self> {
        let mut key = key.to_vec();

        if let Some(id) = id { key.push(id); }

        let mut buffer = Vec::new();
        let mut result = Self { 
            key: key.into_boxed_slice(), 
            value: Default::default(), 
        };

        match ciborium::into_writer(&value, &mut buffer) {
            Ok(()) => {
                result.value = buffer.into_boxed_slice();
                Ok(result)
            },
            Err(err) => Err(result.error(0, err.to_string())),
        }
    }

    pub fn read_state<S: StateBase>(&self) -> Result<S> {
        ciborium::from_reader(Cursor::new(&self.value))
        .map_err(|err| self.error(0, err.to_string()))
    }

    pub fn get_id(&self, depth: usize) -> Option<MessageDataId> { 
        self.key.get(depth).copied()
    }

    pub fn error(
        &self, 
        depth: usize,
        message: impl AsRef<str>,
    ) -> NodeError {
        MessageError { 
            depth: depth as MessageDataDepth, 
            key: self.key.clone(), 
            value: self.value.clone(), 
            message: message.as_ref().to_owned(), 
        }.into()
    }
}

impl TryFrom<MessageData> for Vec<u8> {    
    type Error = NodeError;

    fn try_from(value: MessageData) -> Result<Self> {        
        let mut buffer = Vec::new();

        match ciborium::into_writer(&value, &mut buffer) {
            Ok(()) => Ok(buffer),
            Err(err) => Err(value.error(0, err.to_string())),
        }
    }
}

impl TryFrom<Vec<u8>> for MessageData {    
    type Error = NodeError;

    fn try_from(value: Vec<u8>) -> Result<Self> {      
        ciborium::from_reader(Cursor::new(value))
        .map_err(|err| NodeError::Text(err.to_string()))
    }
}

impl From<anyhow::Result<Vec<u8>>> for MessageData {    
    fn from(value: anyhow::Result<Vec<u8>>) -> Self {      
        value.unwrap().try_into().unwrap()
    }
}

impl TryFrom<MessageData> for String 
{    
    type Error = NodeError;

    fn try_from(value: MessageData) -> Result<Self> {       
        Ok(String::from_utf8(value.try_into()?)?)
    }
}

impl TryFrom<String> for MessageData {    
    type Error = NodeError;

    fn try_from(value: String) -> Result<Self> {      
        value.into_bytes().try_into()
    }
}

impl From<anyhow::Result<String>> for MessageData {   
    fn from(value: anyhow::Result<String>) -> Self {      
        value.unwrap().try_into().unwrap()
    }
}
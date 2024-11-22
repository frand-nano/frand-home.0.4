use std::{error::Error, io::Cursor};
use std::fmt::{self, Debug};
use serde::{Deserialize, Serialize};
use crate::result::{ComponentError, Result};

use super::StateBase;

pub type MessageDataId = u32;
pub type MessageDataKey = Box<[MessageDataId]>;
pub type MessageDataDepth = u32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageError {
    pub depth: MessageDataDepth,
    pub key: MessageDataKey,
    pub value: Box<[u8]>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    key: MessageDataKey,
    value: Box<[u8]>,
}

pub trait MessageBase: Debug + Clone + Sized {
    type State: StateBase;
    
    fn deserialize(depth: usize, data: MessageData) -> Self;
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for MessageError {}

impl MessageData {
    pub fn key(&self) -> &MessageDataKey { &self.key }

    pub fn new<S: StateBase>(
        key: &MessageDataKey, 
        id: Option<MessageDataId>, 
        value: &S,
    ) -> Result<Self> {
        let mut key = key.to_vec();

        if let Some(id) = id { key.push(id); }

        let mut buffer = Vec::new();
        let mut result = Self { 
            key: key.into_boxed_slice(), 
            value: Default::default(), 
        };

        match ciborium::into_writer(value, &mut buffer) {
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
    ) -> ComponentError {
        MessageError { 
            depth: depth as MessageDataDepth, 
            key: self.key.clone(), 
            value: self.value.clone(), 
            message: message.as_ref().to_owned(), 
        }.into()
    }
}
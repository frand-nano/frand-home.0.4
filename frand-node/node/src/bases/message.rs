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
    pub ids: MessageDataKey,
    pub value: Box<[u8]>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    ids: MessageDataKey,
    value: Box<[u8]>,
}

pub trait MessageBase: Debug + Clone + Sized {
    type State: StateBase;
    
    fn deserialize(depth: usize, data: MessageData) -> Result<Self>;
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for MessageError {}

impl MessageData {
    pub fn ids(&self) -> &MessageDataKey { &self.ids }

    pub fn serialize<S: StateBase>(
        ids: &MessageDataKey, 
        id: Option<MessageDataId>, 
        value: &S,
    ) -> Result<Self> {
        let mut ids = ids.to_vec();

        if let Some(id) = id { ids.push(id); }

        let mut buffer = Vec::new();
        let mut result = Self { 
            ids: ids.into_boxed_slice(), 
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

    pub fn deserialize<S: StateBase>(self) -> Result<S> {
        ciborium::from_reader(Cursor::new(&self.value))
        .map_err(|err| self.error(0, err.to_string()))
    }

    pub fn get_id(&self, depth: usize) -> Option<MessageDataId> { 
        self.ids.get(depth).copied()
    }

    pub fn error(
        self, 
        depth: usize,
        message: impl AsRef<str>,
    ) -> ComponentError {
        MessageError { 
            depth: depth as MessageDataDepth, 
            ids: self.ids, 
            value: self.value, 
            message: message.as_ref().to_owned(), 
        }.into()
    }
}
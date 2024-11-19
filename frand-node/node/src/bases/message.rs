use std::io::Cursor;
use std::fmt::Debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use crate::result::{ComponentError, Result};

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
    depth: MessageDataDepth,
    ids: MessageDataKey,
    value: Box<[u8]>,
}

pub trait MessageBase: Debug + Clone + Sized {
    fn deserialize(data: MessageData) -> Result<Self>;
}

impl MessageData {
    pub fn serialize<V: Serialize>(
        ids: &MessageDataKey, 
        id: Option<MessageDataId>, 
        value: &V,
    ) -> Result<Self> {
        let mut ids = ids.to_vec();

        if let Some(id) = id { ids.push(id); }

        let mut buffer = Vec::new();
        let mut result = Self { 
            depth: 0,
            ids: ids.into_boxed_slice(), 
            value: Default::default(), 
        };

        match ciborium::into_writer(value, &mut buffer) {
            Ok(()) => {
                result.value = buffer.into_boxed_slice();
                Ok(result)
            },
            Err(err) => Err(result.error(err.to_string())),
        }
    }

    pub fn deserialize<V: DeserializeOwned>(self) -> Result<V> {
        ciborium::from_reader(Cursor::new(&self.value))
        .map_err(|err| self.error(err.to_string()))
    }

    pub fn pop_id(&mut self) -> Option<MessageDataId> { 
        match self.ids.get(self.depth as usize) {
            Some(id) => {
                self.depth += 1;
                Some(*id)
            },
            None => None,
        }
    }

    pub fn error(
        self, 
        message: impl AsRef<str>,
    ) -> ComponentError {
        ComponentError::Message(
            MessageError { 
                depth: self.depth, 
                ids: self.ids, 
                value: self.value, 
                message: message.as_ref().to_owned(), 
            }
        )
    }
}
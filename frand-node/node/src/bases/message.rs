use std::io::Cursor;
use std::fmt::Debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageError {
    pub depth: usize,
    pub ids: Vec<usize>,
    pub value: Vec<u8>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageData {
    depth: usize,
    ids: Vec<usize>,
    value: Vec<u8>,
}

pub trait MessageBase: Debug + Clone + Sized {
    fn id_count() -> usize;
    fn new_inner(id: usize, data: MessageData) -> Result<Self, MessageError>;

    fn new(mut data: MessageData) -> Result<Self, MessageError> {
        match data.pop_id() {
            Some(id) if id < Self::id_count() => Self::new_inner(id, data),
            Some(_) => Err(data.error(
                "MessageBase::new() id is not in range",
            )),
            None => Err(data.error(
                "MessageBase::new() data has no more id",
            )),
        }     
    }
}

impl MessageData {
    pub fn new<V: Serialize>(ids: &Vec<usize>, value: &V) -> Result<Self, MessageError> {
        let mut result = Self { 
            depth: 0,
            ids: ids.to_owned(), 
            value: Vec::new(), 
        };

        match ciborium::into_writer(value, &mut result.value) {
            Ok(()) => Ok(result),
            Err(err) => Err(result.error(err.to_string())),
        }
    }

    pub fn read<V: DeserializeOwned>(self) -> Result<V, MessageError> {
        ciborium::from_reader(Cursor::new(&self.value))
        .map_err(|err| self.error(err.to_string()))
    }

    pub fn pop_id(&mut self) -> Option<usize> { 
        match self.ids.get(self.depth) {
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
    ) -> MessageError {
        MessageError { 
            depth: self.depth, 
            ids: self.ids, 
            value: self.value, 
            message: message.as_ref().to_owned(), 
        }
    }
}
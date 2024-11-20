use std::{error::Error, fmt, sync::mpsc::SendError};

use crate::bases::{MessageData, MessageError};

pub type Result<T, E = ComponentError> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum ComponentError {
    Text(String),
    Message(MessageError),
    Send(SendError<MessageData>),
    Anyhow(anyhow::Error),
}

impl fmt::Display for ComponentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComponentError::Text(err) => write!(f, "{}", err),
            ComponentError::Message(err) => write!(f, "{:#?}", err),
            ComponentError::Send(err) => write!(f, "{}", err),
            ComponentError::Anyhow(err) => write!(f, "{}", err),
        }
    }
}

impl Error for ComponentError {}

impl From<String> for ComponentError {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for ComponentError {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl From<MessageError> for ComponentError {
    fn from(value: MessageError) -> Self {
        Self::Message(value)
    }
}

impl From<SendError<MessageData>> for ComponentError {
    fn from(value: SendError<MessageData>) -> Self {
        Self::Send(value)
    }
}

impl From<anyhow::Error> for ComponentError {
    fn from(value: anyhow::Error) -> Self {
        Self::Anyhow(value)
    }
}
use std::{error::Error, fmt, string::FromUtf8Error};
use crossbeam::channel::SendError;
use serde::{Deserialize, Serialize};
use crate::bases::{Packet, NodeDepth, NodeKey};

pub type Result<T, E = NodeError> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum NodeError {
    Text(String),
    Message(MessageError),
    Send(SendError<Packet>),
    Poison(String),
    FromUtf8(FromUtf8Error),
    Anyhow(anyhow::Error),
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeError::Text(err) => write!(f, "{}", err),
            NodeError::Message(err) => write!(f, "{:#?}", err),
            NodeError::Send(err) => write!(f, "{}", err),
            NodeError::Poison(err) => write!(f, "{}", err),
            NodeError::FromUtf8(err) => write!(f, "{}", err),
            NodeError::Anyhow(err) => write!(f, "{}", err),
        }
    }
}

impl Error for NodeError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageError {
    pub depth: NodeDepth,
    pub key: NodeKey,
    pub state: Box<[u8]>,
    pub message: String,
}

impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for MessageError {}

impl From<String> for NodeError {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for NodeError {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

impl From<MessageError> for NodeError {
    fn from(value: MessageError) -> Self {
        Self::Message(value)
    }
}

impl From<SendError<Packet>> for NodeError {
    fn from(value: SendError<Packet>) -> Self {
        Self::Send(value)
    }
}

impl From<FromUtf8Error> for NodeError {
    fn from(value: FromUtf8Error) -> Self {
        Self::FromUtf8(value)
    }
}

impl From<anyhow::Error> for NodeError {
    fn from(value: anyhow::Error) -> Self {
        Self::Anyhow(value)
    }
}
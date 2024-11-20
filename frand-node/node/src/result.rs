use std::{error::Error, fmt};

use crate::bases::MessageError;

pub type Result<T, E = ComponentError> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum ComponentError {
    Text(String),
    Message(MessageError),
    Anyhow(anyhow::Error),
}

impl fmt::Display for ComponentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComponentError::Text(err) => write!(f, "Error: {}", err),
            ComponentError::Message(err) => write!(f, "Error: {:#?}", err),
            ComponentError::Anyhow(err) => write!(f, "Error: {}", err),
        }
    }
}

impl Error for ComponentError {}
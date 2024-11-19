use crate::bases::message::MessageError;

pub type Result<T, E = ComponentError> = core::result::Result<T, E>;

pub enum ComponentError {
    Text(String),
    Message(MessageError),
    Anyhow(anyhow::Error),
}
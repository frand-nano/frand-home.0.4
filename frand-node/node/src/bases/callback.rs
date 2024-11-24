use std::{fmt::Debug, sync::Arc};
use crossbeam::channel::Sender;
use crate::{*, result::Result};

#[derive(Clone)]
pub enum CallbackSender {
    Callback(Arc<dyn Fn(Payload) -> Result<()>>),
    Sender(Sender<Payload>),
    None,
}

impl Debug for CallbackSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Callback(_) => f.write_str("Callback(Arc<dyn Fn(Payload) -> Result<()>>)"),
            Self::Sender(sender) => f.debug_tuple("Sender").field(sender).finish(),
            Self::None => write!(f, "None"),
        }
    }
}

impl CallbackSender {
    pub fn callback<U>(update: U) -> Self
    where U: 'static + Fn(Payload) -> Result<()>
    {
        Self::Callback(Arc::new(update))
    }

    pub fn send(&self, message: Payload) -> Result<()> {
        Ok(match self {
            Self::Callback(callback) => (callback)(message)?,
            Self::Sender(callback) => callback.send(message)?,
            Self::None => (),
        })
    }
}
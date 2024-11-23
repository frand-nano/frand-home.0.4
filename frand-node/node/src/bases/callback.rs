use std::{cell::RefCell, fmt::Debug, marker::PhantomData, sync::Arc};
use crossbeam::channel::Sender;
use super::{
    message::{MessageData, MessageDataId, MessageDataKey}, 
    state::StateBase, 
    ContainerCallback, 
};
use crate::result::Result;

#[derive(Debug, Clone)]
pub enum CallbackSender {
    Callback(ContainerCallback),
    Sender(Sender<MessageData>),
    None,
}

impl CallbackSender {
    pub fn callback<U>(update: U) -> Self
    where U: 'static + Fn(MessageData) -> Result<()>
    {
        Self::Callback(ContainerCallback(Arc::new(update)))
    }

    fn send(&self, message: MessageData) -> Result<()> {
        Ok(match self {
            Self::Callback(callback) => (callback)(message)?,
            Self::Sender(sender) => sender.send(message)?,
            Self::None => (),
        })
    }
}

pub trait Emitter<S: StateBase> {
    fn depth(&self) -> usize;
    fn emit(&self, state: S);
}

#[derive(Clone, Debug)]
pub struct Callback<S: StateBase> {
    depth: usize,
    key: MessageDataKey,
    sender: RefCell<CallbackSender>,    
    _phantom: PhantomData<S>,  
}

impl<S: StateBase> PartialEq for Callback<S> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<S: StateBase> Emitter<S> for Callback<S> {
    fn depth(&self) -> usize { self.depth }

    fn emit(&self, state: S) {
        self.sender.borrow().send(
            MessageData::new(&self.key, None, state)
            .unwrap_or_else(|err| panic!("Callback::emit() deserialize Err({err})"))
        )
        .unwrap_or_else(|err| match err {
            crate::result::NodeError::Send(err) => {
                log::debug!("close sender. reason: {err}");
                *self.sender.borrow_mut() = CallbackSender::None;
            },
            _ => panic!("{err}"),
        })
    }
}

impl<S: StateBase> Callback<S> {
    pub fn new(
        sender: &CallbackSender,     
        mut key: Vec<MessageDataId>,
        id: Option<MessageDataId>, 
    ) -> Self {
        if let Some(id) = id { key.push(id); }

        Self { 
            depth: key.len(),
            key: key.into_boxed_slice(),
            sender: RefCell::new(sender.clone()),
            _phantom: Default::default(),
        }
    }

    pub fn reset_sender(&self, sender: &CallbackSender) {
        *self.sender.borrow_mut() = sender.clone();
    }
}

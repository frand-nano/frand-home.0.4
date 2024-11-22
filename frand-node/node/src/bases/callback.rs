use std::{cell::RefCell, fmt::Debug, marker::PhantomData};
use crossbeam::channel::Sender;
use super::{
    message::{MessageData, MessageDataId, MessageDataKey}, 
    state::StateBase, 
    ProcessorCallback, 
};
use crate::result::Result;

#[derive(Debug, Clone)]
pub enum CallbackSender {
    Callback(ProcessorCallback),
    Sender(Sender<MessageData>),
    None,
}

impl CallbackSender {
    fn send(&self, message: MessageData) -> Result<()> {
        Ok(match self {
            Self::Callback(callback) => (callback)(message)?,
            Self::Sender(sender) => sender.send(message)?,
            Self::None => (),
        })
    }
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

    pub fn depth(&self) -> usize { self.depth }

    pub fn emit(&self, state: &S) {
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

    pub fn reset_sender(&self, sender: &CallbackSender) {
        *self.sender.borrow_mut() = sender.clone();
    }
}

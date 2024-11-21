use std::{cell::RefCell, fmt::Debug, marker::PhantomData};
use crossbeam::channel::Sender;
use super::{message::{MessageData, MessageDataId, MessageDataKey}, state::StateBase, ProcessorCallback};
use crate::result::Result;

#[derive(Clone)]
pub enum CallbackSender {
    Callback(ProcessorCallback),
    Sender(Sender<MessageData>),
}

impl CallbackSender {
    fn send(&self, message: MessageData) -> Result<()> {
        Ok(match self {
            Self::Callback(callback) => (callback)(message.clone())?,
            Self::Sender(sender) => sender.send(message)?,
        })
    }
}

#[derive(Clone)]
pub struct Callback<S: StateBase> {
    ids: MessageDataKey,
    sender: RefCell<CallbackSender>,    
    __phantom: PhantomData<S>,  
}

impl<S: StateBase> Debug for Callback<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Callback")
        .field("ids", &self.ids)
        .finish()
    }
}

impl<S: StateBase> PartialEq for Callback<S> {
    fn eq(&self, other: &Self) -> bool {
        self.ids == other.ids
    }
}

impl<S: StateBase> Callback<S> {
    pub fn new(
        sender: &CallbackSender,     
        mut ids: Vec<MessageDataId>,
        id: Option<MessageDataId>, 
    ) -> Self {
        if let Some(id) = id { ids.push(id); }

        Self { 
            ids: ids.into_boxed_slice(),
            sender: RefCell::new(sender.clone()),
            __phantom: Default::default(),
        }
    }

    pub fn reset_sender(&self, sender: &CallbackSender) {
        *self.sender.borrow_mut() = sender.clone();
    }

    pub fn emit(&self, state: &S) -> Result<()> {
        Ok(self.sender.borrow().send(
            MessageData::serialize(&self.ids, None, state)?
        )?)
    }
}

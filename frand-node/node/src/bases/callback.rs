use std::{cell::RefCell, fmt::Debug, marker::PhantomData};
use crossbeam::channel::Sender;
use super::{message::{MessageData, MessageDataId, MessageDataKey}, state::StateBase};
use crate::result::Result;

#[derive(Clone)]
pub struct Callback<S: StateBase> {
    ids: MessageDataKey,
    callback: RefCell<Sender<MessageData>>,    
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
        callback: &Sender<MessageData>,     
        mut ids: Vec<MessageDataId>,
        id: Option<MessageDataId>, 
    ) -> Self {
        if let Some(id) = id { ids.push(id); }

        Self { 
            ids: ids.into_boxed_slice(),
            callback: RefCell::new(callback.clone()),
            __phantom: Default::default(),
        }
    }

    pub fn reset_callback(&self, callback: &Sender<MessageData>) {
        *self.callback.borrow_mut() = callback.clone();
    }

    pub fn emit(&self, state: &S) -> Result<()> {
        Ok(self.callback.borrow().send(
            MessageData::serialize(&self.ids, None, state)?
        )?)
    }
}

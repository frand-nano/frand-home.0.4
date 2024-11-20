use std::{fmt::Debug, marker::PhantomData, sync::mpsc::Sender};
use super::{message::{MessageData, MessageDataId, MessageDataKey}, state::StateBase};
use crate::result::Result;

#[derive(Clone)]
pub struct Callback<S: StateBase> {
    ids: MessageDataKey,
    callback: Sender<MessageData>,    
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
            callback: callback.clone(),
            __phantom: Default::default(),
        }
    }

    pub fn emit(&self, state: &S) -> Result<()> {
        Ok(self.callback.send(
            MessageData::serialize(&self.ids, None, state)?
        )?)
    }
}

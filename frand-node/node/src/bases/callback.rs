use std::{fmt::Debug, marker::PhantomData, rc::Rc};
use super::{context::CreationContext, message::MessageData, state::StateBase};
use crate::result::Result;

#[derive(Clone)]
pub struct Callback<S: StateBase> {
    ids: Vec<usize>,
    callback: Rc<dyn Fn(Result<MessageData>)>,    
    _phantom: PhantomData<S>,  
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
        context: &CreationContext,     
        mut ids: Vec<usize>,
        id: Option<usize>, 
    ) -> Self {
        if let Some(id) = id { ids.push(id); }

        Self { 
            ids,
            callback: context.callback().to_owned(),
            _phantom: Default::default(),
        }
    }

    pub fn emit(&self, state: &S) {
        (self.callback)(MessageData::serialize(&self.ids, None, state))
    }
}

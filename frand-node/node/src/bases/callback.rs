use std::{marker::PhantomData, rc::Rc};
use anyhow::Result;
use super::{context::Context, message::ComponentMessageData, state::State};

pub struct Callback<S: State> {
    ids: Vec<usize>,
    callback: Rc<dyn Fn(Result<ComponentMessageData>)>,    
    _phantom: PhantomData<S>,  
}

impl<S: State> Callback<S> {
    pub fn new(
        context: &Context,     
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
        (self.callback)(ComponentMessageData::new(&self.ids, state))
    }
}

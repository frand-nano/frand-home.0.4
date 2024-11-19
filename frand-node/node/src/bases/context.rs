use std::rc::Rc;
use super::message::MessageData;
use crate::result::Result;

pub struct CreationContext {
    callback: Rc<dyn Fn(Result<MessageData>)>, 
}

impl CreationContext {
    pub fn new(
        callback: Rc<dyn Fn(Result<MessageData>)>, 
    ) -> Self {
        Self { callback }
    }

    pub fn callback(&self) -> &Rc<dyn Fn(Result<MessageData>)> { 
        &self.callback 
    }
}
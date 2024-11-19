use std::rc::Rc;
use super::message::{MessageData, MessageError};

pub struct Context {
    callback: Rc<dyn Fn(Result<MessageData, MessageError>)>, 
}

impl Context {
    pub fn new(
        callback: Rc<dyn Fn(Result<MessageData, MessageError>)>, 
    ) -> Self {
        Self { callback }
    }

    pub fn callback(&self) -> &Rc<dyn Fn(Result<MessageData, MessageError>)> { 
        &self.callback 
    }
}
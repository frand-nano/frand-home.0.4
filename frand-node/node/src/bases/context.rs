use std::rc::Rc;
use super::message::MessageData;

pub struct CreationContext {
    callback: Rc<dyn Fn(MessageData)>, 
}

impl CreationContext {
    pub fn new(
        callback: Rc<dyn Fn(MessageData)>, 
    ) -> Self {
        Self { callback }
    }

    pub fn callback(&self) -> &Rc<dyn Fn(MessageData)> { 
        &self.callback 
    }
}
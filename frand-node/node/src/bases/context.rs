use std::rc::Rc;
use anyhow::Result;
use super::message::ComponentMessageData;

pub struct Context {
    callback: Rc<dyn Fn(Result<ComponentMessageData>)>, 
}

impl Context {
    pub fn new(
        callback: Rc<dyn Fn(Result<ComponentMessageData>)>, 
    ) -> Self {
        Self { callback }
    }

    pub fn callback(&self) -> &Rc<dyn Fn(Result<ComponentMessageData>)> { &self.callback }
}
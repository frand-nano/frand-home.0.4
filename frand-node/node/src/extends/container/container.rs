use std::ops::{Deref, DerefMut};

use bases::CallbackSender;
use crate::*;

pub struct Container<S: StateBase> {     
    node: S::Node,     
    callback: CallbackSender,
}

impl<S: StateBase> Deref for Container<S> {
    type Target = S::Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl<S: StateBase> DerefMut for Container<S> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.node }
}

impl<S: 'static + StateBase> Container<S> {
    pub fn new<U>(update: U) -> Self 
    where U: 'static + Fn(MessageData)
    {
        let callback = CallbackSender::callback(move |data| {      
            (update)(data);
            Ok(())
        });

        Self { 
            node: S::Node::new(&callback, vec![], None), 
            callback,
        }      
    }

    pub fn new_with<U>(node: &S::Node, update: U) -> Self 
    where U: 'static + Fn(MessageData)
    {
        let mut result = Self::new(update);
        node.set_callback(&result.callback);
        result.node = node.clone();
        result
    }
}

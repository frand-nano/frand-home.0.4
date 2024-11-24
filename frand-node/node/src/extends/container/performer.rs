use std::ops::{Deref, DerefMut};
use crossbeam::channel::Sender;
use bases::CallbackSender;
use crate::*;
use super::Processor;

pub struct Performer<S: StateBase> {     
    node: S::Node,     
    callback: CallbackSender,
    inbound_tx: Sender<Payload>, 
}

impl<S: StateBase> Deref for Performer<S> {
    type Target = S::Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl<S: StateBase> DerefMut for Performer<S> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.node }
}

impl<S: 'static + StateBase> Performer<S> {
    pub fn new<U>(update: U) -> Self 
    where U: 'static + Fn(&S::Node, S::Message, Payload)
    {
        let (callback, inbound_tx) = Processor::<S, U>::new_callback(update);

        Self { 
            node: S::Node::new(&callback, vec![], None), 
            callback,
            inbound_tx,
        }      
    }

    pub fn new_with<U>(node: &S::Node, update: U) -> Self 
    where U: 'static + Fn(&S::Node, S::Message, Payload)
    {
        let mut result = Self::new(update);
        node.set_callback(&result.callback);
        result.node = node.clone();
        result
    }

    pub fn send(&self, payload: Payload) {
        self.inbound_tx.send(payload)
        .expect("Failed to send payload through inbound_tx: the callback might have been dropped or modified")
    }
}

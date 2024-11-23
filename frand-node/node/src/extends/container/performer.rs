use std::ops::{Deref, DerefMut};
use crossbeam::channel::Sender;
use bases::{CallbackSender, Emitter};
use crate::*;
use super::Processor;

pub struct Performer<S: StateBase> {     
    node: S::Node,     
    callback: CallbackSender,
    inbound_tx: Sender<MessageData>, 
}

impl<S: StateBase> Deref for Performer<S> {
    type Target = S::Node;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl<S: StateBase> DerefMut for Performer<S> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.node }
}

impl<S: StateBase> ContainerBase<S> for Performer<S> {

}

impl<S: 'static + StateBase> Performer<S> {
    pub fn inbound_tx(&self) -> &Sender<MessageData> { &self.inbound_tx }

    pub fn new<U>(update: U) -> Self 
    where U: 'static + Fn(&S::Node, S::Message, MessageData)
    {
        let (callback, inbound_tx) = Processor::<S, U>::new_callback(update);

        Self { 
            node: S::Node::new(&callback, vec![], None), 
            callback,
            inbound_tx,
        }      
    }

    pub fn new_with<U>(node: &S::Node, update: U) -> Self 
    where U: 'static + Fn(&S::Node, S::Message, MessageData)
    {
        let mut result = Self::new(update);
        node.reset_sender(&result.callback);
        result.node = node.clone();
        result
    }
}

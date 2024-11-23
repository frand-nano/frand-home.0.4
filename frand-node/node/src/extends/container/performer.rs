use std::ops::{Deref, DerefMut};
use crossbeam::channel::Sender;
use bases::{CallbackSender, Emitter};
use crate::*;
use super::Processor;

pub struct Performer<S: StateBase> {     
    node: S::Node,     
    callback: CallbackSender,
    input: Sender<MessageData>, 
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
    pub fn input(&self) -> &Sender<MessageData> { &self.input }

    pub fn new<U>(update: U) -> Self 
    where U: 'static + Fn(&S::Node, S::Message, MessageData)
    {
        let (callback, input) = Processor::<S, U>::new_callback(update);

        Self { 
            node: S::Node::new(&callback, vec![], None), 
            callback,
            input,
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

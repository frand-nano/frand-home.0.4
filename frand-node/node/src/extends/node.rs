use std::{borrow::BorrowMut, ops::Deref};
use bases::{ElementBase, PayloadId, Reporter};
use crate::*;

#[derive(Debug, Clone)]
pub struct Node<S: StateBase + MessageBase> {
    emitter: Emitter,    
    value: S,
}

impl<S: StateBase + MessageBase> Deref for Node<S> {
    type Target = Emitter;
    fn deref(&self) -> &Self::Target { &self.emitter }
}

impl<S: StateBase + MessageBase> Node<S> {
    pub fn value(&self) -> &S { &self.value }
}

impl<S: StateBase + MessageBase> Default for Node<S> {
    fn default() -> Self {
        Self::new(&Reporter::None, vec![], None)
    }
}

impl<S: StateBase + MessageBase> PartialEq for Node<S> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<S: StateBase + MessageBase> ElementBase for Node<S> {
    type State = S;
    type Node = Self;
    type Message = S;
}

impl<S: StateBase + MessageBase> NodeBase for Node<S> {      
    fn new(
        reporter: &Reporter,   
        mut key: Vec<PayloadId>,
        id: Option<PayloadId>,  
    ) -> Self {
        if let Some(id) = id { key.push(id); }

        Self { 
            emitter: Emitter::new(reporter, key),
            value: S::default(), 
        }
    }  

    fn set_reporter(&self, reporter: &Reporter) {
        self.deref().borrow_mut().set_reporter(reporter);
    }
}

impl<S: StateBase + MessageBase> Stater<S> for Node<S> {    
    fn apply(&mut self, payload: &Payload) {
        let depth = self.depth();
        match payload.get_id(depth) {
            Some(_) => Err(payload.error(depth, "unknown id")),
            None => Ok(self.apply_state(payload.read_state())),
        }
        .unwrap_or_else(|err| panic!("Node<S>::apply() deserialize Err({err})"));
    }

    fn apply_state(&mut self, state: S) {
        self.value = state;
    }
}
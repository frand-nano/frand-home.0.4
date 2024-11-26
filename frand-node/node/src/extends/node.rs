use std::sync::Arc;
use bases::{ElementBase, PayloadId};
use crate::*;

#[derive(Debug, Clone)]
pub struct Node<S: StateBase + MessageBase> {
    emitter: Emitter<Self>,    
    value: S,
}

impl<S: StateBase + MessageBase> Node<S> {
    pub fn value(&self) -> &S { &self.value }
}

impl<S: StateBase + MessageBase> Default for Node<S> {
    fn default() -> Self { Self::new() }
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
    fn new_child(
        mut key: Vec<PayloadId>,
        id: Option<PayloadId>,  
    ) -> Self {
        if let Some(id) = id { key.push(id); }

        Self { 
            emitter: Emitter::new(key),
            value: S::default(), 
        }
    }  

    fn emit(&self, state: Self::State) {
        self.emitter.emit(state);
    }

    fn emit_payload(&self, payload: Payload) {
        self.emitter.emit_payload(payload);
    }

    fn set_callback<F>(&self, callback: &Arc<F>)  
    where F: 'static + Fn(Payload) {
        self.emitter.set_callback(callback.clone());
    }

    fn activate<F>(&self, callback: F) -> &Self 
    where F: 'static + Fn(Payload) {
        self.emitter.set_callback(Arc::new(callback));
        self
    }

    fn fork<F>(&self, callback: F) -> Self 
    where F: 'static + Fn(Payload) {
        let result = self.clone();
        result.emitter.set_callback(Arc::new(callback));        
        result
    }

    fn inject(&self, process: fn(&Self, Payload, Self::Message)) -> &Self {
        self.emitter.set_process(process);
        self
    }

    fn to_message(&self, payload: &Payload) -> Self::Message {
        let depth = payload.key().len();
        match payload.get_id(depth) {
            Some(_) => Err(payload.error(depth, "unknown id")),
            None => Ok(payload.read_state::<S>()),
        }     
        .unwrap_or_else(|err| panic!("{}::to_message() Err({err})", stringify!(Node<S>)))
    }
}

impl<S: StateBase + MessageBase> Stater<S> for Node<S> {    
    fn apply(&mut self, payload: &Payload) {
        let depth = self.emitter.depth();
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
use std::{ops::Deref, sync::Arc};
use bases::{ElementBase, PayloadId};
use crate::*;

#[derive(Debug)]
pub struct Node<S: StateBase + MessageBase> {
    emitter: Emitter<Self>,    
    value: S,
}

impl<S: StateBase + MessageBase> Clone for Node<S> {
    fn clone(&self) -> Self {
        log::debug!("Node<S>::clone value:{:?}", self.value);
        Self { 
            emitter: self.emitter.clone(), 
            value: self.value.clone(), 
        }
    }
}

impl<S: StateBase + MessageBase> Deref for Node<S> {
    type Target = S;
    fn deref(&self) -> &Self::Target { &self.value }
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
        let depth = self.emitter.depth();
        match payload.get_id(depth) {
            Some(_) => Err(payload.error(depth, "unknown id")),
            None => Ok(self.emitter.emit_payload(payload)),
        }
        .unwrap_or_else(|err| panic!("Node<S>::emit_payload() deserialize Err({err})"));
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

    fn inject(&self, process: fn(&Self, &Payload, Self::Message)) -> &Self {
        self.emitter.set_process(process);
        self
    }

    fn call_process(&self, depth: usize, payload: &Payload) {
        match payload.get_id(depth) {
            Some(_) => Err(payload.error(depth, "unknown id")),
            None => Ok(self.emitter.call_process(self, depth, payload)),
        }     
        .unwrap_or_else(|err| panic!("{}::call_process() Err({err})", stringify!(Node<S>)))
    }
}

impl<S: StateBase + MessageBase> Stater<S> for Node<S> {  
    fn apply(&mut self, state: S) {
        self.value = state;
    }

    fn apply_payload(&mut self, payload: &Payload) {
        let depth = self.emitter.depth();
        match payload.get_id(depth) {
            Some(_) => Err(payload.error(depth, "unknown id")),
            None => Ok(self.apply(payload.read_state())),
        }
        .unwrap_or_else(|err| panic!("Node<S>::apply_payload() deserialize Err({err})"));
    }
}
use std::{ops::Deref, sync::Arc};
use bases::{ElementBase, NodeId};
use crate::*;

#[derive(Debug, PartialEq)]
pub struct Node<S: StateBase + MessageBase> {
    emitter: Emitter<Self, S>,    
}

impl<S: StateBase + MessageBase> Clone for Node<S> {
    fn clone(&self) -> Self {
        log::debug!("Node<S>::clone");
        Self { 
            emitter: self.emitter.clone(), 
        }
    }
}

impl<S: StateBase + MessageBase> Deref for Node<S> {
    type Target = S;
    fn deref(&self) -> &Self::Target { self.emitter.value() }
}

impl<S: StateBase + MessageBase> Default for Node<S> {
    fn default() -> Self { Self::new() }
}

impl<S: StateBase + MessageBase> ElementBase for Node<S> {
    type State = S;
    type Node = Self;
    type Message = S;
}

impl<S: StateBase + MessageBase> NodeBase for Node<S> {      
    fn new_child(
        mut key: Vec<NodeId>,
        id: Option<NodeId>,  
    ) -> Self {
        if let Some(id) = id { key.push(id); }

        Self { 
            emitter: Emitter::new(key),
        }
    }  

    fn emit(&self, state: Self::State) {
        self.emitter.emit(state);
    }

    fn emit_packet(&self, packet: Packet) {
        self.emitter.emit_packet(packet);
    }

    fn set_callback<F>(&self, callback: &Arc<F>)  
    where F: 'static + Fn(Packet) {
        self.emitter.set_callback(callback.clone());
    }

    fn activate<F>(&self, callback: F) -> &Self 
    where F: 'static + Fn(Packet) {
        self.emitter.set_callback(Arc::new(callback));
        self
    }

    fn fork<F>(&self, callback: F) -> Self 
    where F: 'static + Fn(Packet) {
        let result = self.clone();
        result.emitter.set_callback(Arc::new(callback));        
        result
    }

    fn inject(&self, process: fn(&Self, &Packet, Self::Message)) -> &Self {
        self.emitter.set_process(process);
        self
    }

    fn call_process(&self, depth: usize, packet: &Packet) {
        match packet.get_id(depth) {
            Some(_) => Err(packet.error(depth, "unknown id")),
            None => Ok(self.emitter.call_process(self, depth, packet)),
        }     
        .unwrap_or_else(|err| panic!("{}::call_process() Err({err})", stringify!(Node<S>)))
    }
}

impl<S: StateBase + MessageBase> Stater<S> for Node<S> {  
    fn apply(&mut self, state: S) {
        *self.emitter.value_mut() = state;
    }

    fn apply_packet(&mut self, packet: &Packet) {
        let depth = self.emitter.depth();
        match packet.get_id(depth) {
            Some(_) => Err(packet.error(depth, "unknown id")),
            None => Ok(self.apply(packet.read_state())),
        }
        .unwrap_or_else(|err| panic!("Node<S>::apply_packet() deserialize Err({err})"));
    }
}
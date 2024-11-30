use std::{future::Future, ops::Deref};
use bases::{ElementBase, NodeId, Reporter};
use crate::*;

#[derive(Debug, PartialEq)]
pub struct Node<S: StateBase + MessageBase> {
    emitter: Emitter<S>,    
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
    fn default() -> Self { Self::new(vec![], None, Reporter::None) }
}

impl<S: StateBase + MessageBase> ElementBase for Node<S> {
    type State = S;
    type Node = Self;
    type Message = S;
}

impl<S: StateBase + MessageBase> NodeBase for Node<S> {      
    fn new(
        mut key: Vec<NodeId>,
        id: Option<NodeId>,  
        reporter: Reporter,
    ) -> Self {
        if let Some(id) = id { key.push(id); }

        Self { 
            emitter: Emitter::new(key, reporter),
        }
    }  

    fn emit(&self, state: Self::State) {
        self.emitter.emit(state);
    }

    fn emit_packet(&self, packet: Packet) {
        self.emitter.emit_packet(packet);
    }

    fn emit_future<Fu>(&self, future: Fu) 
    where Fu: 'static + Future<Output = Self::State> {
        self.emitter.emit_future(future);
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
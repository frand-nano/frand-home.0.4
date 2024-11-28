use std::{cell::OnceCell, fmt::Debug, sync::Arc};
use bases::{NodeId, NodeKey};
use crate::*;

pub struct Emitter<N: NodeBase, V: StateBase = ()> {
    depth: usize,
    key: NodeKey,
    value: V,
    callback: OnceCell<Arc<dyn Fn(Packet)>>,    
    process: OnceCell<fn(&N, &Packet, N::Message)>,    
}

impl<N: NodeBase + Clone, V: StateBase> PartialEq for Emitter<N, V> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<N: NodeBase + Clone, V: StateBase> Clone for Emitter<N, V> {
    fn clone(&self) -> Self {
        log::debug!("Emitter<N>::clone key:{:?}, value:{:?}", self.key, self.value);
        Self { 
            depth: self.depth.clone(), 
            key: self.key.clone(), 
            value: self.value.clone(),
            callback: self.callback.clone(), 
            process: self.process.clone(), 
        }
    }
}

impl<N: NodeBase, V: StateBase> Debug for Emitter<N, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Emitter")
        .field("depth", &self.depth)
        .field("key", &self.key)
        .field("value", &self.value)
        .field("callback", &"OnceCell<Option<Arc<dyn Fn(Packet)>>>")
        .field("process", &"OnceCell<Option<fn(&N, Packet, N::Message)>>")
        .finish()
    }
}

impl<N: NodeBase, V: StateBase> Default for Emitter<N, V> {
    fn default() -> Self {
        Self { 
            depth: Default::default(), 
            key: Default::default(), 
            value: Default::default(), 
            callback: OnceCell::new(),
            process: OnceCell::new(),
        }
    }
}

impl<N: NodeBase, V: StateBase> Emitter<N, V> {
    pub fn depth(&self) -> usize { self.depth }
    pub fn value(&self) -> &V { &self.value }
    pub fn value_mut(&mut self) -> &mut V { &mut self.value }

    pub fn new(
        key: Vec<NodeId>,
    ) -> Self {
        log::debug!("Emitter<N>::new {:?}", key);
        Self { 
            depth: key.len(),
            key: key.into_boxed_slice(),   
            value: V::default(),            
            callback: OnceCell::new(),
            process: OnceCell::new(),      
        }
    }  

    pub fn set_callback(&self, callback: Arc<dyn Fn(Packet)>) {        
        if let Err(_) = self.callback.set(callback) {
            panic!("Calling set_callback multiple times is not allowed.");
        }
    }

    pub fn set_process(&self, process: fn(&N, &Packet, N::Message)) {
        if let Err(err) = self.process.set(process) {
            panic!("Calling set_process multiple times is not allowed. Err({:#?})", err);
        }
    }

    pub fn call_process(&self, node: &N, depth: usize, packet: &Packet) {
        if let Some(process) = self.process.get() {
            let message = N::Message::from_packet(depth, packet);
            (process)(node, packet, message);
        }
    }

    pub fn emit<S: StateBase>(&self, state: S) {
        self.emit_packet(Packet::new(self.key.clone(), state));
    }

    pub fn emit_packet(&self, packet: Packet) {
        if let Some(callback) = self.callback.get() {
            (callback)(packet);
        }
    }
}
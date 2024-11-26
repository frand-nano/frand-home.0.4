use std::{cell::OnceCell, fmt::Debug, sync::Arc};
use bases::{PayloadId, PayloadKey};
use crate::*;

pub struct Emitter<N: NodeBase, V: StateBase = ()> {
    depth: usize,
    key: PayloadKey,
    value: V,
    callback: OnceCell<Arc<dyn Fn(Payload)>>,    
    process: OnceCell<fn(&N, &Payload, N::Message)>,    
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
        .field("callback", &"OnceCell<Option<Arc<dyn Fn(Payload)>>>")
        .field("process", &"OnceCell<Option<fn(&N, Payload, N::Message)>>")
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
        key: Vec<PayloadId>,
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

    pub fn set_callback(&self, callback: Arc<dyn Fn(Payload)>) {        
        if let Err(_) = self.callback.set(callback) {
            panic!("Calling set_callback multiple times is not allowed.");
        }
    }

    pub fn set_process(&self, process: fn(&N, &Payload, N::Message)) {
        if let Err(err) = self.process.set(process) {
            panic!("Calling set_process multiple times is not allowed. Err({:#?})", err);
        }
    }

    pub fn call_process(&self, node: &N, depth: usize, payload: &Payload) {
        if let Some(process) = self.process.get() {
            let message = N::Message::from_payload(depth, payload);
            (process)(node, payload, message);
        }
    }

    pub fn emit<S: StateBase>(&self, state: S) {
        self.emit_payload(Payload::new(self.key.clone(), state));
    }

    pub fn emit_payload(&self, payload: Payload) {
        if let Some(callback) = self.callback.get() {
            (callback)(payload);
        }
    }
}
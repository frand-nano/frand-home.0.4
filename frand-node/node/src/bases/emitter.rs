use std::{cell::RefCell, fmt::Debug, sync::Arc};
use bases::{PayloadId, PayloadKey};
use crate::*;

pub struct Emitter<N: NodeBase> {
    depth: usize,
    key: PayloadKey,
    callback: RefCell<Option<Arc<dyn Fn(Payload)>>>,    
    process: RefCell<Option<fn(&N, &Payload, N::Message)>>,    
}

impl<N: NodeBase + Clone> Clone for Emitter<N> {
    fn clone(&self) -> Self {
        log::debug!("Emitter<N>::clone {:?}", self.key);
        Self { 
            depth: self.depth.clone(), 
            key: self.key.clone(), 
            callback: self.callback.clone(), 
            process: self.process.clone(), 
        }
    }
}

impl<N: NodeBase> Debug for Emitter<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Emitter")
        .field("depth", &self.depth)
        .field("key", &self.key)
        .field("callback", &"RefCell<Option<Arc<dyn Fn(Payload)>>>")
        .field("process", &"RefCell<Option<fn(&N, Payload, N::Message)>>")
        .finish()
    }
}

impl<N: NodeBase> Default for Emitter<N> {
    fn default() -> Self {
        Self { 
            depth: Default::default(), 
            key: Default::default(), 
            callback: RefCell::new(None),
            process: RefCell::new(None),
        }
    }
}

impl<N: NodeBase> Emitter<N> {
    pub fn depth(&self) -> usize { self.depth }

    pub fn new(
        key: Vec<PayloadId>,
    ) -> Self {
        log::debug!("Emitter<N>::new {:?}", key);
        Self { 
            depth: key.len(),
            key: key.into_boxed_slice(),            
            callback: RefCell::new(None),
            process: RefCell::new(None),      
        }
    }  

    pub fn overwrite_callback(&self, callback: Arc<dyn Fn(Payload)>) {
        *self.callback.borrow_mut() = Some(callback);
    }

    pub fn set_callback(&self, callback: Arc<dyn Fn(Payload)>) {
        if self.callback.borrow().is_some() {
            panic!("
                Calling set_callback multiple times is not allowed. 
                Instead, use overwrite_callback.
                If you have called activate, use fork or inject instead.
            ");
        } else {
            *self.callback.borrow_mut() = Some(callback);
        }
    }

    pub fn overwrite_process(&self, process: fn(&N, &Payload, N::Message)) {
        *self.process.borrow_mut() = Some(process);
    }

    pub fn set_process(&self, process: fn(&N, &Payload, N::Message)) {
        if self.process.borrow().is_some() {
            panic!("
                Calling set_process multiple times is not allowed. 
                Instead, use overwrite_process.
            ");
        } else {
            *self.process.borrow_mut() = Some(process);
        }
    }

    pub fn call_process(&self, node: &N, depth: usize, payload: &Payload) {
        if let Some(process) = self.process.borrow().as_ref() {
            let message = N::Message::from_payload(depth, payload);
            (process)(node, payload, message);
        }
    }

    pub fn emit<S: StateBase>(&self, state: S) {
        self.emit_payload(Payload::new(self.key.clone(), state));
    }

    pub fn emit_payload(&self, payload: Payload) {
        if let Some(callback) = self.callback.borrow().as_ref() {
            (callback)(payload);
        }
    }
}
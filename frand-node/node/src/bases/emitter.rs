use std::{cell::RefCell, fmt::Debug, sync::Arc};
use bases::{PayloadId, PayloadKey};
use crate::*;

#[derive(Clone)]
pub struct Emitter<N: NodeBase> {
    depth: usize,
    key: PayloadKey,
    callback: RefCell<Arc<dyn Fn(Payload)>>,    
    process: RefCell<Option<fn(&N, Payload, N::Message)>>,    
}

impl<N: NodeBase> Debug for Emitter<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Emitter")
        .field("depth", &self.depth)
        .field("key", &self.key)
        .field("callback", &"RefCell<Arc<dyn Fn(Payload)>>")
        .field("process", &"RefCell<Option<fn(&N, Payload, N::Message)>>")
        .finish()
    }
}

impl<N: NodeBase> Default for Emitter<N> {
    fn default() -> Self {
        Self { 
            depth: Default::default(), 
            key: Default::default(), 
            callback: RefCell::new(Arc::new(|_| ())),
            process: RefCell::new(None),
        }
    }
}

impl<N: NodeBase> Emitter<N> {
    pub fn depth(&self) -> usize { self.depth }

    pub fn new(
        key: Vec<PayloadId>,
    ) -> Self {
        Self { 
            depth: key.len(),
            key: key.into_boxed_slice(),            
            callback: RefCell::new(Arc::new(|_| ())),
            process: RefCell::new(None),      
        }
    }  

    pub fn set_callback(&self, callback: Arc<dyn Fn(Payload)>) {
        *self.callback.borrow_mut() = callback;
    }

    pub fn set_process(&self, process: fn(&N, Payload, N::Message)) {
        if self.process.borrow().is_some() {
            log::warn!("Process overwrite actions are disallowed");
        }
        *self.process.borrow_mut() = Some(process);
    }

    pub fn emit<S: StateBase>(&self, state: S) {
        self.emit_payload(Payload::new(self.key.clone(), state));
    }

    pub fn emit_payload(&self, payload: Payload) {
        (self.callback.borrow())(payload);
    }
}
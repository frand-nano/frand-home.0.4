use std::{cell::{Ref, RefCell}, fmt::Debug, sync::Arc};
use bases::{PayloadId, PayloadKey};
use crossbeam::channel::Sender;
use crate::*;

#[derive(Debug, Default, Clone)]
pub struct Emitter {
    depth: usize,
    key: PayloadKey,
    reporter: RefCell<Reporter>,    
}

#[derive(Default, Clone)]
pub enum Reporter {
    Callback(Arc<dyn Fn(Payload)>),
    Sender(Sender<Payload>),
    #[default] None,
}

impl Debug for Reporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Callback(_) => f.write_str("Arc<dyn Fn(Payload)>"),
            Self::Sender(sender) => f.debug_tuple("Sender").field(sender).finish(),
            Self::None => f.debug_tuple("None").finish(),
        }
    }
}

impl Emitter {
    pub fn depth(&self) -> usize { self.depth }
    pub fn reporter(&self) -> Ref<Reporter> { self.reporter.borrow() }

    pub fn new(
        reporter: Reporter,   
        key: Vec<PayloadId>,
    ) -> Self {
        Self { 
            depth: key.len(),
            key: key.into_boxed_slice(),
            reporter: RefCell::new(reporter),
        }
    }  

    pub fn set_reporter(&self, reporter: Reporter) {
        *self.reporter.borrow_mut() = reporter;
    }

    pub fn emit<S: StateBase>(&self, state: S) {
        self.reporter.borrow().emit(Payload::new(&self.key, None, state));
    }
}

impl Reporter {
    pub fn emit(&self, payload: Payload) {
        match self {
            Reporter::Callback(callback) => (callback)(payload),
            Reporter::Sender(sender) => sender.send(payload).unwrap(),
            Reporter::None => {},
        }
    }
}
use std::cell::{Ref, RefCell};
use bases::{CallbackSender, ElementBase, PayloadId, PayloadKey};
use result::NodeError;
use crate::*;

#[derive(Debug, Clone)]
pub struct Node<S: StateBase + MessageBase> {
    depth: usize,
    key: PayloadKey,
    callback: RefCell<CallbackSender>,    
    value: S,
}

impl<S: StateBase + MessageBase> Node<S> {
    pub fn value(&self) -> &S { &self.value }
}

impl<S: StateBase + MessageBase> Default for Node<S> {
    fn default() -> Self {
        Self::new(&CallbackSender::None, vec![], None)
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
        callback: &CallbackSender,   
        mut key: Vec<PayloadId>,
        id: Option<PayloadId>,  
    ) -> Self {
        if let Some(id) = id { key.push(id); }

        Self { 
            depth: key.len(),
            key: key.into_boxed_slice(),
            callback: RefCell::new(callback.clone()),
            value: S::default(), 
        }
    }  
}

impl<S: StateBase + MessageBase> Emitter<S> for Node<S> {
    fn depth(&self) -> usize { self.depth }
    fn callback(&self) -> Ref<CallbackSender> { self.callback.borrow() }

    fn set_callback(&self, callback: &CallbackSender) { 
        *self.callback.borrow_mut() = callback.clone();        
    }

    fn emit(&self, state: S) {
        self.callback.borrow().send(
            Payload::new(&self.key, None, state)
        )
        .unwrap_or_else(|err| match err {
            NodeError::Send(err) => {
                log::debug!("close callback. reason: {err}");
                *self.callback.borrow_mut() = CallbackSender::None;
            },
            _ => panic!("{err}"),
        })
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
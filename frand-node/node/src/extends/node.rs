use std::cell::RefCell;
use bases::{CallbackSender, MessageDataId, MessageDataKey};
use result::NodeError;
use crate::*;

#[derive(Debug, Clone)]
pub struct Node<V: StateBase + MessageBase> {
    depth: usize,
    key: MessageDataKey,
    callback: RefCell<CallbackSender>,    
    value: V,
}

impl<V: StateBase + MessageBase> Node<V> {
    pub fn value(&self) -> &V { &self.value }
}

impl<V: StateBase + MessageBase> Default for Node<V> {
    fn default() -> Self {
        Self::new(&CallbackSender::None, vec![], None)
    }
}

impl<V: StateBase + MessageBase> PartialEq for Node<V> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<V: StateBase + MessageBase> NodeBase<V> for Node<V> {    
    fn new(
        callback: &CallbackSender,   
        mut key: Vec<MessageDataId>,
        id: Option<MessageDataId>,  
    ) -> Self {
        if let Some(id) = id { key.push(id); }

        Self { 
            depth: key.len(),
            key: key.into_boxed_slice(),
            callback: RefCell::new(callback.clone()),
            value: V::default(), 
        }
    }
}

impl<V: StateBase + MessageBase> Emitter<V> for Node<V> {
    fn depth(&self) -> usize { self.depth }

    fn set_callback(&self, callback: &CallbackSender) { 
        *self.callback.borrow_mut() = callback.clone();        
    }

    fn emit(&self, state: V) {
        self.callback.borrow().send(
            MessageData::new(&self.key, None, state)
            .unwrap_or_else(|err| panic!("Callback::emit() deserialize Err({err})"))
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

impl<V: StateBase + MessageBase> Stater<V> for Node<V> {    
    fn apply(&mut self, data: &MessageData) {
        let depth = self.depth();
        match data.get_id(depth) {
            Some(_) => Err(data.error(depth, "unknown id")),
            None => data.read_state().map(|state| self.apply_state(state)),
        }
        .unwrap_or_else(|err| panic!("Node<V>::apply() deserialize Err({err})"));
    }

    fn apply_state(&mut self, state: V) {
        self.value = state;
    }
}
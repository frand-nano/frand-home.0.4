use crate::bases::{Callback, CallbackSender, MessageBase, MessageData, MessageDataId, NodeBase, StateBase};

#[derive(Debug, Clone, PartialEq)]
pub struct Node<V: StateBase + MessageBase> {
    value: V,
    callback: Callback<V>,
}

impl<V: StateBase + MessageBase> Default for Node<V> {
    fn default() -> Self {
        Self::new(&CallbackSender::None, vec![], None)
    }
}

impl<V: StateBase + MessageBase> Node<V> {
    pub fn value(&self) -> &V { &self.value }
}

impl<V: StateBase + MessageBase> NodeBase for Node<V> {    
    type State = V;

    fn emit(&self, state: &V) { self.callback.emit(state) }

    fn new(
        sender: &CallbackSender,   
        mut key: Vec<MessageDataId>,
        id: Option<MessageDataId>,  
    ) -> Self {
        if let Some(id) = id { key.push(id); }

        Self { 
            value: V::default(), 
            callback: Callback::new(sender, key, Some(0)), 
        }
    }

    fn reset_sender(&self, sender: &CallbackSender) { 
        self.callback.reset_sender(sender); 
    }

    fn apply(&mut self, data: &MessageData) {
        let depth = self.callback.depth()-1;
        match data.get_id(depth) {
            Some(0) => data.read_state().map(|state| self.apply_state(state)),
            Some(_) => Err(data.error(depth, "unknown id")),
            None => Err(data.error(depth, "data has no more id")),
        }
        .unwrap_or_else(|err| panic!("Node<V>::apply() deserialize Err({err})"));
    }

    fn apply_state(&mut self, state: V) {
        self.value = state;
    }
}
use crate::{
    bases::{Callback, CallbackSender, MessageBase, MessageData, MessageDataId, NodeBase, StateBase}, 
    result::Result,
};

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

    fn emit(&self, state: &V) -> Result<()> {
        self.callback.emit(state)
    }

    fn new(
        sender: &CallbackSender,   
        mut ids: Vec<MessageDataId>,
        id: Option<MessageDataId>,  
    ) -> Self {
        if let Some(id) = id { ids.push(id); }

        Self { 
            value: V::default(), 
            callback: Callback::new(sender, ids, Some(0)), 
        }
    }

    fn reset_sender(&self, sender: &CallbackSender) {
        self.callback.reset_sender(sender);
    }

    fn apply(&mut self, data: MessageData) -> Result<()> {
        match data.next() {
            (Some(0), data) => Ok(self.__apply_state(data.deserialize()?)),
            (Some(_), data) => Err(data.error(
                format!("Node<V>::apply() unknown id"),
            )),
            (None, data) => Err(data.error(
                format!("Node<V>::apply() data has no more id"),
            )),
        }     
    }

    #[doc(hidden)]
    fn __apply_state(&mut self, state: V) {
        self.value = state;
    }
}
use crate::bases::{
    callback::Callback, context::CreationContext, message::MessageDataId, node::NodeBase, state::StateBase
};

use super::message::NodeMessage;

#[derive(Debug, Clone, PartialEq)]
pub struct Node<S: StateBase> {
    state: S,
    callback: Callback<S>,
}

impl<S: StateBase> NodeBase<S> for Node<S> {
    type Message = NodeMessage<S>;

    fn new(
        context: &CreationContext,   
        mut ids: Vec<MessageDataId>,
        id: Option<MessageDataId>,  
    ) -> Self {
        if let Some(id) = id { ids.push(id); }

        Self { 
            state: S::default(), 
            callback: Callback::new(context, ids, Some(0)), 
        }
    }

    fn emit(&self, state: &S) {
        self.callback.emit(state);
    }
}
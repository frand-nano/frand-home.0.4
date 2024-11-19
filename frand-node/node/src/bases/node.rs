use super::{context::CreationContext, message::{MessageBase, MessageDataId}, state::StateBase};

pub trait NodeBase<S: StateBase>: Clone {    
    type Message: MessageBase;

    fn new(
        context: &CreationContext,     
        ids: Vec<MessageDataId>,
        id: Option<MessageDataId>,
    ) -> Self;

    fn emit(&self, state: &S);
}
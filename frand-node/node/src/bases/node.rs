use super::{context::CreationContext, message::MessageBase, state::StateBase};

pub trait NodeBase<S: StateBase>: Clone {    
    type Message: MessageBase;

    fn new(
        context: &CreationContext,     
        ids: Vec<usize>,
        id: Option<usize>,
    ) -> Self;

    fn emit(&self, state: &S);
}
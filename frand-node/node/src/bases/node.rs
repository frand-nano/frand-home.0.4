use super::{context::Context, message::MessageBase, state::StateBase};

pub trait NodeBase<S: StateBase> {    
    type Message: MessageBase;

    fn new(
        context: &Context,     
        ids: Vec<usize>,
        id: Option<usize>,
    ) -> Self;

    fn emit(&self, state: &S);
}
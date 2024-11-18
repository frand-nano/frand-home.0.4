use super::{context::Context, state::State};

pub trait NodeBase<S: State> {    
    fn new(
        context: &Context,     
        ids: Vec<usize>,
        id: Option<usize>,
    ) -> Self;

    fn emit(&self, state: &S);
}
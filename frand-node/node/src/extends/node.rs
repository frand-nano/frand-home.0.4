use crate::bases::{
    callback::Callback,
    context::Context, 
    message::MESSAGE_ID_SELF, 
    node::NodeBase, 
    state::State,
};

pub struct Node<S: State> {
    callback: Callback<S>,
    state: S,
}

impl<S: State> PartialEq for Node<S> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl<S: State> NodeBase<S> for Node<S> {
    fn new(
        context: &Context,   
        mut ids: Vec<usize>,
        id: Option<usize>,  
    ) -> Self {
        if let Some(id) = id { ids.push(id); }

        Self { 
            callback: Callback::new(
                context, 
                ids, 
                Some(MESSAGE_ID_SELF), 
            ), 
            state: S::default(), 
        }
    }

    fn emit(&self, state: &S) {
        self.callback.emit(state)
    }
}
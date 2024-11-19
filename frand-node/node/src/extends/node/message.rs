use crate::bases::{
    message::{self, MessageData, MessageError}, 
    state::StateBase,
};

#[derive(Debug, Clone)]
pub enum NodeMessage<S: StateBase> {
    State(S),
}

impl<S: StateBase> message::MessageBase for NodeMessage<S> {
    fn id_count() -> usize { 1 }

    fn new_inner(id: usize, data: MessageData) -> Result<Self, MessageError> {
        Ok(match id {
            0 => Self::State(data.read()?),
            _ => Err(data.error("NodeMessage<S> unknown id"))?,
        })
    }
}
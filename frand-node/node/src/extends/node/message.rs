use crate::{
    result::Result,
    bases::{
        message::{MessageBase, MessageData}, 
        state::StateBase,
    },
};

#[derive(Debug, Clone)]
pub enum NodeMessage<S: StateBase> {
    State(S),
}

impl<S: StateBase> MessageBase for NodeMessage<S> {
    fn deserialize(mut data: MessageData) -> Result<Self> {
        match data.pop_id() {
            Some(0) => Ok(Self::State(data.deserialize()?)),
            Some(_) => Err(data.error(
                format!("NodeMessage<S>::deserialize() unknown id"),
            )),
            None => Err(data.error(
                format!("NodeMessage<S>::deserialize() data has no more id"),
            )),
        }     
    }
}
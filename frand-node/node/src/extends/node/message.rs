use crate::bases::{
    message::{self, MessageData, MessageError}, 
    state::StateBase,
};

#[derive(Debug, Clone)]
pub enum NodeMessage<S: StateBase> {
    State(S),
}

impl<S: StateBase> message::MessageBase for NodeMessage<S> {
    fn deserialize(mut data: MessageData) -> Result<Self, MessageError> {
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
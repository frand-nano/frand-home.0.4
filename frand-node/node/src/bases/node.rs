use std::fmt::Debug;
use super::{message::{MessageData, MessageDataId}, state::StateBase, CallbackSender};

pub trait NodeBase: Debug + Clone + Sized {    
    type State: StateBase; 

    fn emit(&self, state: &Self::State);

    fn new(
        sender: &CallbackSender,     
        key: Vec<MessageDataId>,
        id: Option<MessageDataId>,
    ) -> Self;

    fn reset_sender(&self, sender: &CallbackSender);
    fn apply(&mut self, data: &MessageData);
    fn apply_state(&mut self, state: Self::State);
}
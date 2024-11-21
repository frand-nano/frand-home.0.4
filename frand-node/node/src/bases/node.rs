use std::fmt::Debug;
use super::{message::{MessageData, MessageDataId}, state::StateBase, CallbackSender};
use crate::result::Result;

pub trait NodeBase: Debug + Clone + Sized {    
    type State: StateBase; 

    fn emit(&self, state: &Self::State) -> Result<()>;

    fn new(
        sender: &CallbackSender,     
        ids: Vec<MessageDataId>,
        id: Option<MessageDataId>,
    ) -> Self;

    fn reset_sender(&self, sender: &CallbackSender);
    fn apply(&mut self, data: MessageData) -> Result<()>;

    #[doc(hidden)]
    fn __apply_state(&mut self, state: Self::State);
}
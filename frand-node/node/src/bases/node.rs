use std::fmt::Debug;
use crossbeam::channel::Sender;
use super::{message::{MessageData, MessageDataId}, state::StateBase};
use crate::result::Result;

pub trait NodeBase: Debug + Clone {    
    type State: StateBase;

    fn emit(&self, state: &Self::State) -> Result<()>;

    fn new(
        callback: &Sender<MessageData>,     
        ids: Vec<MessageDataId>,
        id: Option<MessageDataId>,
    ) -> Self;

    #[doc(hidden)]
    fn __apply(&mut self, data: MessageData) -> Result<()>;

    #[doc(hidden)]
    fn __apply_state(&mut self, state: Self::State);
}
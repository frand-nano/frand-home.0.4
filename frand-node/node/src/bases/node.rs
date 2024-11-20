use std::{fmt::Debug, rc::Rc};
use super::{message::{MessageBase, MessageData, MessageDataId}, state::StateBase};
use crate::result::Result;

pub trait NodeBase<S: StateBase>: Debug + Clone {    
    type Message: MessageBase;

    fn new(
        callback: &Rc<dyn Fn(MessageData)>,     
        ids: Vec<MessageDataId>,
        id: Option<MessageDataId>,
    ) -> Self;

    fn emit(&self, state: &S) -> Result<()>;

    #[doc(hidden)]
    fn __apply(&mut self, data: MessageData) -> Result<()>;

    #[doc(hidden)]
    fn __apply_state(&mut self, state: S);
}
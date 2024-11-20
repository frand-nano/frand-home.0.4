use std::{fmt::Debug, rc::Rc};
use super::{message::{MessageData, MessageDataId}, state::StateBase};
use crate::result::Result;

pub trait NodeBase<S: StateBase>: Debug + Clone {    
    fn emit(&self, state: &S) -> Result<()>;

    fn new(
        callback: &Rc<dyn Fn(MessageData)>,     
        ids: Vec<MessageDataId>,
        id: Option<MessageDataId>,
    ) -> Self;

    #[doc(hidden)]
    fn __apply(&mut self, data: MessageData) -> Result<()>;

    #[doc(hidden)]
    fn __apply_state(&mut self, state: S);
}
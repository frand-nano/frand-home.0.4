use std::fmt::Debug;
use super::{message::{MessageData, MessageDataId}, state::StateBase, CallbackSender};

pub trait NodeBase<S: StateBase>: Debug + Clone + Sized + PartialEq + Emitter<S> + Stater<S> {    
    fn new(
        callback: &CallbackSender,     
        key: Vec<MessageDataId>,
        id: Option<MessageDataId>,
    ) -> Self;
}

pub trait Emitter<S: StateBase> {
    fn depth(&self) -> usize;
    fn set_callback(&self, callback: &CallbackSender);
    fn emit(&self, state: S);
}

pub trait Stater<S: StateBase> {
    fn apply(&mut self, message: &MessageData);
    fn apply_state(&mut self, state: S);

    fn apply_messages<I>(&mut self, messages: I) 
    where 
        I: Iterator<Item = MessageData>,
        I::Item: AsRef<MessageData>,
    {
        for message in messages {
            self.apply(message.as_ref());
        }
    }
}

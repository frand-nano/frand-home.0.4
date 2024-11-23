use std::{fmt::Debug, ops::Deref};
use super::{message::{MessageData, MessageDataId}, state::StateBase, Callback, CallbackSender};

pub trait NodeBase<S: StateBase>: Debug + Clone + Sized + Deref<Target = Callback<S>> + Stater<S> {    
    fn new(
        sender: &CallbackSender,     
        key: Vec<MessageDataId>,
        id: Option<MessageDataId>,
    ) -> Self;

    fn reset_sender(&self, sender: &CallbackSender);
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

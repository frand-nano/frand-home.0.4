use std::cell::Ref;
use super::{message::{Payload, PayloadId}, state::StateBase, CallbackSender, ElementBase};

pub trait NodeBase: ElementBase + Emitter<Self::State> + Stater<Self::State> {   
    fn new(
        callback: &CallbackSender,     
        key: Vec<PayloadId>,
        id: Option<PayloadId>,
    ) -> Self;
}

pub trait Emitter<S: StateBase> {
    fn depth(&self) -> usize;
    fn callback(&self) -> Ref<CallbackSender>;
    fn set_callback(&self, callback: &CallbackSender);
    fn emit(&self, state: S);
}

pub trait Stater<S: StateBase> {
    fn apply(&mut self, message: &Payload);
    fn apply_state(&mut self, state: S);

    fn apply_messages<I>(&mut self, messages: I) 
    where 
        I: Iterator<Item = Payload>,
        I::Item: AsRef<Payload>,
    {
        for message in messages {
            self.apply(message.as_ref());
        }
    }
}

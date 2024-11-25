use std::ops::Deref;
use super::{message::{Payload, PayloadId}, state::StateBase, ElementBase, Emitter, Reporter};

pub trait NodeBase: ElementBase + Deref<Target = Emitter> + Stater<Self::State> {   
    fn new(
        reporter: &Reporter,     
        key: Vec<PayloadId>,
        id: Option<PayloadId>,
    ) -> Self;

    fn set_reporter(&self, reporter: &Reporter);
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

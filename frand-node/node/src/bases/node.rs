use std::sync::Arc;
use super::{message::{Payload, PayloadId}, state::StateBase, ElementBase};

pub trait NodeBase: ElementBase + Stater<Self::State> {   
    fn new() -> Self { Self::new_child(vec![], None) }

    fn new_child(  
        key: Vec<PayloadId>,
        id: Option<PayloadId>,
    ) -> Self;

    fn new_activate<F>(callback: F) -> Self
    where F: 'static + Fn(Payload) {
        let result = Self::new();
        result.activate(callback);
        result
    }

    fn emit(&self, state: Self::State);
    fn emit_payload(&self, payload: Payload);

    fn set_callback<F>(&self, callback: &Arc<F>)  
    where F: 'static + Fn(Payload);

    fn activate<F>(&self, callback: F) -> &Self 
    where F: 'static + Fn(Payload);

    fn fork<F>(&self, callback: F) -> Self 
    where F: 'static + Fn(Payload);

    fn inject(&self, process: fn(&Self, &Payload, Self::Message)) -> &Self;

    fn call_process(&self, depth: usize, payload: &Payload);
}

pub trait Stater<S: StateBase> {
    fn apply(&mut self, state: S);
    fn apply_payload(&mut self, payload: &Payload);

    fn apply_payloads<I>(&mut self, payloads: I) 
    where 
        I: Iterator<Item = Payload>,
        I::Item: AsRef<Payload>,
    {
        for payload in payloads {
            self.apply_payload(payload.as_ref());
        }
    }
}

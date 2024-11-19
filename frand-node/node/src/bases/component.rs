use std::sync::mpsc::{Receiver, Sender};
use crate::result::Result;
use super::{message::MessageData, state::StateBase};

pub trait ComponentBase<S: StateBase> {
    fn node(&self) -> &S::Node;
    fn input_tx(&self) -> &Sender<MessageData>;
    fn output_rx(&mut self) -> Result<Receiver<Result<MessageData>>>;
    
    fn new() -> Self;
    fn perform(&mut self);
}

pub trait Component<S: StateBase>: ComponentBase<S> {
    
}
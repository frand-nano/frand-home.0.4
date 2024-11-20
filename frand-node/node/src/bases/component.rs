use std::sync::mpsc::{Receiver, Sender};
use crate::result::Result;
use super::{message::MessageData, state::StateBase, Performer};

pub trait ComponentBase<S: StateBase> {
    fn node(&self) -> &S::Node;
    fn performer(&self) -> &Performer;
    
    fn input_tx(&self) -> &Sender<MessageData> { self.performer().input_tx() }    
    fn output_rx(&mut self) -> &mut Option<Receiver<Result<MessageData>>>;
    fn perform(&mut self) -> Result<()>;    
    
    fn new() -> Self;
}

pub trait Component<S: StateBase>: ComponentBase<S> {
    fn control(node: &S::Node, message: S::Message) -> Result<()>;
}
use std::sync::mpsc::{Receiver, Sender};
use crate::result::Result;
use super::{message::MessageData, state::StateBase};

pub trait ComponentBase<S: StateBase> {
    fn node(&self) -> &S::Node;
    fn input_tx(&self) -> &Sender<MessageData>;
    fn output_rx(&mut self) -> Receiver<Result<MessageData>>;
    
    fn new(control: Box<dyn Fn(&S::Node, S::Message) -> Result<()>>) -> Self;
    fn perform(&mut self) -> Result<()>;    
}

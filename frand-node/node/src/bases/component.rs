use std::sync::mpsc::{Receiver, Sender};
use crate::result::Result;
use super::{message::MessageData, state::StateBase, MessageBase, NodeBase, Performer};

pub trait ComponentBase {
    type State: StateBase;
    type Node: NodeBase;
    type Message: MessageBase;

    fn node(&self) -> &Self::Node;
    fn performer(&self) -> &Performer;
    
    fn input_tx(&self) -> &Sender<MessageData> { self.performer().input_tx() }    
    fn output_rx(&mut self) -> &mut Option<Receiver<Result<MessageData>>>;
    fn perform(&mut self) -> Result<()>;    
    
    fn new() -> Self;
}

pub trait Component: ComponentBase {
    fn control(node: &Self::Node, message: Self::Message) -> Result<()>;
}
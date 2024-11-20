use crossbeam::channel::{Receiver, Sender};
use crate::result::Result;
use super::{message::MessageData, state::StateBase, MessageBase, NodeBase};

pub trait ComponentBase {
    type State: StateBase;
    type Node: NodeBase;
    type Message: MessageBase;

    fn node(&self) -> &Self::Node;
    
    fn input_tx(&self) -> &Sender<MessageData>;   
    fn take_output_rx(&mut self) -> Option<Receiver<MessageData>>;
    
    fn new() -> Self;
    fn perform(&mut self) -> Result<()>;    
}

pub trait Component: ComponentBase {
    fn update(node: &Self::Node, message: Self::Message) -> anyhow::Result<()>;
}
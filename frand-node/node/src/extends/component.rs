use std::sync::mpsc::Receiver;
use crate::{
    bases::{ComponentBase, MessageData, Performer, StateBase, NodeBase}, 
    result::Result,
};

pub struct Component<S: StateBase> {    
    node: S::Node,     
    performer: Performer,
    output_rx: Option<Receiver<Result<MessageData>>>,
}

impl<S: StateBase> ComponentBase<S> for Component<S> {
    fn node(&self) -> &S::Node { &self.node }
    fn performer(&self) -> &Performer { &self.performer }
    fn output_rx(&mut self) -> &mut Option<Receiver<Result<MessageData>>> { &mut self.output_rx }

    fn new() -> Self {
        let (performer, output_rx) = Performer::new();

        Self { 
            node: S::Node::new(performer.callback(), vec![], None), 
            performer,
            output_rx: Some(output_rx),
        }
    }
    
    fn perform(&mut self) -> Result<()> {
        todo!()
    }
}
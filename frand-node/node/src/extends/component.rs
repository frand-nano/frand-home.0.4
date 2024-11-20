use std::sync::mpsc::{Receiver, Sender};
use crate::{
    bases::{ComponentBase, MessageData, Performer, StateBase, NodeBase}, 
    result::Result,
};

#[derive(Debug)]
pub struct Component<S: StateBase> {    
    node: S::Node,     
    performer: Performer<S>,
}

impl<S: StateBase> ComponentBase<S> for Component<S> {
    fn node(&self) -> &S::Node { &self.node }
    fn input_tx(&self) -> &Sender<MessageData> { &self.performer.input_tx }    
    fn output_rx(&mut self) -> Receiver<Result<MessageData>> { self.performer.output_rx() }

    fn new(control: Box<dyn Fn(&S::Node, S::Message) -> Result<()>>) -> Self {
        let performer = Performer::new(control);

        Self { 
            node: S::Node::new(performer.callback(), vec![], None), 
            performer,
        }
    }

    fn perform(&mut self) -> Result<()> {        
        self.performer.perform(&mut self.node)
    }
}
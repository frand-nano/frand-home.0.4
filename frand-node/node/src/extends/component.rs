use std::sync::mpsc::{Receiver, Sender};
use crate::{
    bases::{ComponentBase, MessageData, NodeBase, Performer, StateBase}, 
    result::Result,
};

pub struct Component<S: StateBase> {    
    node: S::Node,     
    performer: Performer,
}

impl<S: StateBase> ComponentBase for Component<S> 
where 
Self: crate::bases::Component,
Self: ComponentBase<Node = S::Node>,
{
    type State = S;
    type Node = S::Node;
    type Message = S::Message;

    fn node(&self) -> &Self::Node { &self.node }
    fn input_tx(&self) -> &Sender<MessageData> { self.performer.input_tx() }    
    fn take_output_rx(&mut self) -> Option<Receiver<Result<MessageData>>> { self.performer.take_output_rx() }

    fn new() -> Self {
        let performer = Performer::new();

        Self { 
            node: S::Node::new(performer.callback(), vec![], None), 
            performer,
        }
    }

    fn perform(&mut self) -> Result<()> {
        self.performer.perform::<Self>(&mut self.node)
    }
}
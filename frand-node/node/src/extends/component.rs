use std::sync::mpsc::Receiver;
use crate::{
    bases::{ComponentBase, MessageData, NodeBase, Performer, StateBase}, 
    result::Result,
};

pub struct Component<S: StateBase> {    
    node: S::Node,     
    performer: Performer,
    output_rx: Option<Receiver<Result<MessageData>>>,
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
        self.performer.perform::<Self>(&mut self.node)
    }
}
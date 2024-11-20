use std::sync::mpsc::{Receiver, Sender};
use crate::{
    bases::{ComponentBase, MessageData, Performer, StateBase}, 
    result::Result,
};

pub struct Component<S: StateBase> {   
    performer: Performer<S>,
}

impl<S: StateBase> ComponentBase for Component<S> 
where 
Self: crate::bases::Component,
Self: ComponentBase<Node = S::Node>,
{
    type State = S;
    type Node = S::Node;
    type Message = S::Message;

    fn node(&self) -> &Self::Node { &self.performer.node() }
    fn input_tx(&self) -> &Sender<MessageData> { self.performer.input_tx() }    
    fn take_output_rx(&mut self) -> Option<Receiver<MessageData>> { self.performer.take_output_rx() }
    fn perform(&mut self) -> Result<()> { self.performer.perform::<Self>() }

    fn new() -> Self {
        Self { performer: Performer::new() }
    }
}
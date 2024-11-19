use std::{rc::Rc, sync::mpsc::{channel, Receiver, Sender}};
use crate::{
    bases::{
        component::ComponentBase, 
        context::CreationContext,
        message::MessageData, 
        node::NodeBase, 
        state::StateBase,
    }, 
    result::{ComponentError, Result},
};

use super::Performer;

#[derive(Debug)]
pub struct Component<S: StateBase> {    
    node: S::Node,     
    performer: Performer<S>,
}

impl<S: StateBase> ComponentBase<S> for Component<S> {
    fn node(&self) -> &S::Node { &self.node }
    fn input_tx(&self) -> &Sender<MessageData> { &self.performer.input_tx }

    fn output_rx(&mut self) -> Result<Receiver<Result<MessageData>>> {
        if self.performer.output_tx.is_some() {
            Err(ComponentError::Text(
                format!("Component<S> output_rx has already been created.")
            ))
        } else {
            let (output_tx, output_rx) = channel();
            self.performer.output_tx = Some(output_tx);
            Ok(output_rx)
        }
    }

    fn new(control: Box<dyn Fn(&S::Node, S::Message) -> Result<()>>) -> Self {
        let (performer, node_tx) = Performer::new(control);

        let context = CreationContext::new(Rc::new(move |message| {
            if let Err(err) = node_tx.send(message) {
                log::error!("Component node_tx.send(message) err:{err}");
            }
        }));

        Self { 
            node: S::Node::new(&context, vec![], None), 
            performer,
        }
    }

    fn perform(&mut self) -> Result<()> {        
        self.performer.perform(&mut self.node)
    }
}
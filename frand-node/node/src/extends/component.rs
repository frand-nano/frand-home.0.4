use std::{collections::HashMap, rc::Rc, sync::mpsc::{channel, Receiver, Sender}};
use crate::{
    bases::{component::ComponentBase, context::CreationContext, message::{MessageData, MessageDataKey}, node::NodeBase, state::StateBase}, 
    result::{ComponentError, Result},
};

pub struct Component<S: StateBase> {    
    node: S::Node,     
    node_rx: Receiver<Result<MessageData>>,
    input_tx: Sender<MessageData>,
    input_rx: Receiver<MessageData>,
    output_tx: Option<Sender<Result<MessageData>>>,
    current_messages: HashMap<MessageDataKey, MessageData>,
    next_messages: HashMap<MessageDataKey, MessageData>,
}

impl<S: StateBase> ComponentBase<S> for Component<S> {
    fn node(&self) -> &S::Node { &self.node }
    fn input_tx(&self) -> &Sender<MessageData> { &self.input_tx }

    fn output_rx(&mut self) -> Result<Receiver<Result<MessageData>>> {
        if self.output_tx.is_some() {
            Err(ComponentError::Text(
                format!("Component<S> output_rx has already been created.")
            ))
        } else {
            let (output_tx, output_rx) = channel();
            self.output_tx = Some(output_tx);
            Ok(output_rx)
        }
    }

    fn new() -> Self {
        let (node_tx, node_rx) = channel();

        let context = CreationContext::new(Rc::new(move |message| {
            if let Err(err) = node_tx.send(message) {
                log::error!("Component node_tx.send(message) err:{err}");
            }
        }));

        let (input_tx, input_rx) = channel();

        Self { 
            node: S::Node::new(&context, vec![], None), 
            node_rx,
            input_tx, 
            input_rx, 
            output_tx: None, 
            current_messages: HashMap::new(),
            next_messages: HashMap::new(),
        }
    }

    fn perform(&mut self) {
        
    }
}
use std::{collections::HashSet, sync::mpsc::{channel, Receiver, Sender}};
use crate::{
    bases::{
        message::{MessageBase, MessageData, MessageDataKey}, 
        node::NodeBase, 
    }, 
    result::Result,
};

use super::{Component, StateBase};

pub struct Performer<S: StateBase> {     
    node: S::Node,     
    node_tx: Sender<MessageData>,
    node_rx: Receiver<MessageData>,
    input_tx: Sender<MessageData>,
    input_rx: Receiver<MessageData>,
    output_tx: Sender<MessageData>,
    output_rx: Option<Receiver<MessageData>>,
    performed_messages: HashSet<MessageDataKey>,
    next_messages: Vec<MessageData>,
}

impl<S: StateBase> Performer<S> {
    pub fn node(&self) -> &S::Node { &self.node }
    pub fn input_tx(&self) -> &Sender<MessageData> { &self.input_tx }
    pub fn take_output_rx(&mut self) -> Option<Receiver<MessageData>> { self.output_rx.take() }

    pub fn new() -> Self {
        let (node_tx, node_rx) = channel();
        let (input_tx, input_rx) = channel();
        let (output_tx, output_rx) = channel();

        Self { 
            node: S::Node::new(&node_tx, vec![], None), 
            node_tx,
            node_rx,
            input_tx, 
            input_rx, 
            output_tx, 
            output_rx: Some(output_rx),
            performed_messages: HashSet::new(),
            next_messages: Vec::new(),
        }
    }

    pub fn perform<C: Component>(&mut self) -> Result<()> 
    where C: Component<Node = S::Node>
    {
        let mut messages = Vec::new();
        messages.extend(self.input_rx.try_iter());
        messages.extend(self.node_rx.try_iter());
        messages.append(&mut self.next_messages);

        while !messages.is_empty() {    
            for message in &messages {
                self.node.__apply(message.clone())?;

                if self.output_rx.is_none() {
                    self.output_tx.send(message.clone())?;
                }
            }

            for message in messages.drain(..) {
                self.node_tx.send(message)?;

                loop {
                    if let Ok(message) = self.node_rx.try_recv() {
                        if self.performed_messages.contains(message.ids()) {
                            self.next_messages.push(message);
                        } else {
                            self.performed_messages.insert(message.ids().clone());
        
                            self.node.__apply(message.clone())?;
                            
                            let message = C::Message::deserialize(message)?;
                            C::control(&self.node, message)?;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
            
        self.performed_messages.clear();

        Ok(())
    }
}

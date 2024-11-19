use std::{collections::HashSet, fmt::Debug, sync::mpsc::{channel, Receiver, Sender}};
use crate::{
    bases::{
        message::{MessageBase, MessageData, MessageDataKey}, 
        node::NodeBase, 
        state::StateBase,
    }, 
    result::Result,
};

pub struct Performer<S: StateBase> {    
    pub control: Box<dyn Fn(&S::Node, S::Message) -> Result<()>>,
    pub node_tx: Sender<MessageData>,
    pub node_rx: Receiver<MessageData>,
    pub input_tx: Sender<MessageData>,
    pub input_rx: Receiver<MessageData>,
    pub output_tx: Option<Sender<Result<MessageData>>>,
    pub performed_messages: HashSet<MessageDataKey>,
    pub next_messages: Vec<MessageData>,
}

impl<S: StateBase> Debug for Performer<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Performer")
        .field("node_tx", &self.node_tx)
        .field("node_rx", &self.node_rx)
        .field("input_tx", &self.input_tx)
        .field("input_rx", &self.input_rx)
        .field("output_tx", &self.output_tx)
        .field("performed_messages", &self.performed_messages)
        .field("next_messages", &self.next_messages)
        .finish()
    }
}

impl<S: StateBase> Performer<S> {
    pub fn new(control: Box<dyn Fn(&S::Node, S::Message) -> Result<()>>) -> (Self, Sender<MessageData>) {
        let (node_tx, node_rx) = channel();
        let (input_tx, input_rx) = channel();

        (
            Self { 
                control,
                node_tx: node_tx.clone(),
                node_rx,
                input_tx, 
                input_rx, 
                output_tx: None, 
                performed_messages: HashSet::new(),
                next_messages: Vec::new(),
            },
            node_tx,
        )
    }

    pub fn perform(&mut self, node: &mut S::Node) -> Result<()> {
        let mut messages = Vec::new();
        messages.extend(self.input_rx.try_iter());
        messages.extend(self.node_rx.try_iter());
        messages.append(&mut self.next_messages);

        while !messages.is_empty() {    
            for message in &messages {
                node.__apply(message.clone())?;

                if let Some(output_tx) = &self.output_tx {         
                    if let Err(err) = output_tx.send(Ok(message.clone())) {
                        log::error!("Component output_tx.send(message) err:{err}");
                    }
                }
            }

            for message in messages.drain(..) {
                if let Err(err) = self.node_tx.send(message) {
                    log::error!("Component node_tx.send(message) err:{err}");
                }

                loop {
                    if let Ok(message) = self.node_rx.try_recv() {
                        if self.performed_messages.contains(message.ids()) {
                            self.next_messages.push(message);
                        } else {
                            self.performed_messages.insert(message.ids().clone());
        
                            node.__apply(message.clone())?;
                            
                            let message = S::Message::deserialize_message(message)?;
                            (self.control)(&node, message)?;
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

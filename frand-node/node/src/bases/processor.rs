use std::{collections::HashSet, sync::{Arc, Mutex}};
use crossbeam::channel::{unbounded, Receiver, Sender};

use crate::{
    bases::{
        message::{MessageBase, MessageData, MessageDataKey}, 
        node::NodeBase, CallbackSender, 
    }, 
    result::{ComponentError, Result},
};

use super::{Component, ComponentBase};

pub type ProcessorCallback = Arc<dyn Fn(MessageData) -> Result<()>>;

pub struct Processor<C: ComponentBase> {     
    node: C::Node,    
    node_tx: Sender<MessageData>,
    node_rx: Receiver<MessageData>, 
    input: Receiver<MessageData>,
    output_tx: Sender<MessageData>,
    handled_messages: HashSet<MessageDataKey>,
}

impl<C: ComponentBase> Processor<C> {
    pub fn new() -> (Processor<C>, Sender<MessageData>, Receiver<MessageData>) {
        log::info!("Processor new");

        let (input, input_rx) = unbounded();
        let (node_tx, node_rx) = unbounded();
        let (output_tx, output) = unbounded();

        let sender = CallbackSender::Sender(node_tx.clone());
        let processor = Self { 
            node: C::Node::new(&sender, vec![], None), 
            node_tx,
            node_rx, 
            input: input_rx,
            output_tx,
            handled_messages: HashSet::new(),
        };

        (processor, input, output)
    }
    
    pub fn new_callback() -> (ProcessorCallback, Sender<MessageData>, Receiver<MessageData>) 
    where C: Component
    {
        let (processor, input, output) = Self::new();

        let input_clone = input.clone();
        let processor = Mutex::new(processor);
        let callback = Arc::new(move |message| {
            input_clone.send(message)?;
            processor.lock()
            .map_err(|err| ComponentError::Text(err.to_string()))?
            .process()
        });

        (callback, input, output)
    }

    pub fn process(&mut self) -> Result<()> 
    where C: Component
    {
        for message in self.input.try_iter() {
            self.node_tx.send(message)?;

            while let Ok(message) = self.node_rx.try_recv() {
                if !self.handled_messages.contains(message.ids()) {
                    self.handled_messages.insert(message.ids().clone());
    
                    self.node.apply(message.clone())?;
                    self.output_tx.send(message.clone())?;
                    
                    let message = C::Message::deserialize(message)?;
                    C::update(&self.node, message)?;
                }
            }
                
            self.handled_messages.clear();
        }

        Ok(())
    }
}
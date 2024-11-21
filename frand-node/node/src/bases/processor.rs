use std::{collections::HashSet, sync::{Arc, Mutex}};
use crossbeam::channel::{unbounded, Receiver, Sender};

use crate::{
    bases::{
        message::{MessageBase, MessageData, MessageDataKey}, 
        node::NodeBase, CallbackSender, 
    }, 
    result::{ComponentError, Result},
};

use super::StateBase;

pub type ProcessorCallback = Arc<dyn Fn(MessageData) -> Result<()>>;

pub struct Processor<S: StateBase, U> 
where U: 'static + Fn(&S::Node, S::Message, MessageData) -> anyhow::Result<()>
{     
    node: S::Node,    
    update: U,
    node_tx: Sender<MessageData>,
    node_rx: Receiver<MessageData>, 
    input: Receiver<MessageData>,
    handled_messages: HashSet<MessageDataKey>,
}

impl<S: 'static + StateBase, U> Processor<S, U> 
where U: 'static + Fn(&S::Node, S::Message, MessageData) -> anyhow::Result<()>
{
    pub fn new(update: U) -> (Processor<S, U>, Sender<MessageData>) {
        log::info!("Processor new");

        let (input, input_rx) = unbounded();
        let (node_tx, node_rx) = unbounded();

        let sender = CallbackSender::Sender(node_tx.clone());
        let processor = Self { 
            node: S::Node::new(&sender, vec![], None), 
            update,
            node_tx,
            node_rx, 
            input: input_rx,
            handled_messages: HashSet::new(),
        };

        (processor, input)
    }
    
    pub fn new_callback(update: U) -> (ProcessorCallback, Sender<MessageData>) {
        let (processor, input) = Self::new(update);

        let input_clone = input.clone();
        let processor = Mutex::new(processor);
        let callback = Arc::new(move |data| {
            input_clone.send(data)?;
            processor.lock()
            .map_err(|err| ComponentError::Text(err.to_string()))?
            .process()
        });

        (callback, input)
    }

    pub fn process(&mut self) -> Result<()> {
        for data in self.input.try_iter() {
            self.node_tx.send(data)?;

            while let Ok(data) = self.node_rx.try_recv() {
                if !self.handled_messages.contains(data.ids()) {
                    self.handled_messages.insert(data.ids().clone());
    
                    self.node.apply(data.clone())?;
                    
                    let message = S::Message::deserialize(0, data.clone())?;
                    (self.update)(&self.node, message, data)?;
                }
            }
                
            self.handled_messages.clear();
        }

        Ok(())
    }
}
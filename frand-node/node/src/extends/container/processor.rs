use std::{collections::HashSet, sync::Mutex};
use crossbeam::channel::{unbounded, Receiver, Sender};
use bases::{CallbackSender, MessageDataKey};
use result::NodeError;
use crate::*;

pub struct Processor<S: StateBase, U> 
where U: 'static + Fn(&S::Node, S::Message, MessageData)
{     
    node: S::Node,    
    update: U,
    node_tx: Sender<MessageData>,
    node_rx: Receiver<MessageData>, 
    input: Receiver<MessageData>,
    handled_messages: HashSet<MessageDataKey>,
}

impl<S: 'static + StateBase, U> Processor<S, U> 
where U: 'static + Fn(&S::Node, S::Message, MessageData)
{
    pub fn new(update: U) -> (Processor<S, U>, Sender<MessageData>) {
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
    
    pub fn new_callback(update: U) -> (CallbackSender, Sender<MessageData>) {
        let (processor, input) = Self::new(update);

        let input_clone = input.clone();
        let processor = Mutex::new(processor);
        let callback = CallbackSender::callback(move |data| {
            input_clone.send(data)?;

            processor.lock()
            .map_err(|err| NodeError::Poison(err.to_string()))?
            .process();

            Ok(())
        });

        (callback, input)
    }

    pub fn process(&mut self) {
        for data in self.input.try_iter() {
            self.node_tx.send(data)
            .unwrap_or_else(|err| 
                panic!("Unexpected error while sending message. self.node_tx.send(data): Err({err})")
            );

            while let Ok(data) = self.node_rx.try_recv() {
                if !self.handled_messages.contains(data.key()) {
                    self.handled_messages.insert(data.key().clone());
    
                    self.node.apply(&data);
                    
                    let message = S::Message::deserialize(0, data.clone());
                    (self.update)(&self.node, message, data);
                }
            }
                
            self.handled_messages.clear();
        }
    }
}
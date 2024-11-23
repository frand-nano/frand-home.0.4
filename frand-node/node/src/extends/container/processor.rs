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
    inbound: Receiver<MessageData>,
    handled_messages: HashSet<MessageDataKey>,
}

impl<S: 'static + StateBase, U> Processor<S, U> 
where U: 'static + Fn(&S::Node, S::Message, MessageData)
{
    pub fn new(update: U) -> (Processor<S, U>, Sender<MessageData>) {
        let (inbound_tx, inbound_rx) = unbounded();
        let (node_tx, node_rx) = unbounded();

        let callback = CallbackSender::Sender(node_tx.clone());
        let processor = Self { 
            node: S::Node::new(&callback, vec![], None), 
            update,
            node_tx,
            node_rx, 
            inbound: inbound_rx,
            handled_messages: HashSet::new(),
        };

        (processor, inbound_tx)
    }
    
    pub fn new_callback(update: U) -> (CallbackSender, Sender<MessageData>) {
        let (processor, inbound_tx) = Self::new(update);

        let inbound_tx_clone = inbound_tx.clone();
        let processor = Mutex::new(processor);
        let callback = CallbackSender::callback(move |data| {
            inbound_tx_clone.send(data)?;

            processor.lock()
            .map_err(|err| NodeError::Poison(err.to_string()))?
            .process();

            Ok(())
        });

        (callback, inbound_tx)
    }

    pub fn process(&mut self) {
        for data in self.inbound.try_iter() {
            self.node_tx.send(data)
            .unwrap_or_else(|err| 
                panic!("Unexpected error while sending message. self.node_tx.send(data): Err({err})")
            );

            while let Ok(data) = self.node_rx.try_recv() {
                log::info!("self.node_rx.try_recv() {:?}", data);
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
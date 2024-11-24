use std::{collections::HashSet, sync::Mutex};
use crossbeam::channel::{unbounded, Receiver, Sender};
use bases::{CallbackSender, PayloadKey};
use result::NodeError;
use crate::*;

pub struct Processor<S: StateBase, U> 
where U: 'static + Fn(&S::Node, S::Message, Payload)
{     
    node: S::Node,    
    update: U,
    node_tx: Sender<Payload>,
    node_rx: Receiver<Payload>, 
    inbound: Receiver<Payload>,
    handled_messages: HashSet<PayloadKey>,
}

impl<S: 'static + StateBase, U> Processor<S, U> 
where U: 'static + Fn(&S::Node, S::Message, Payload)
{
    pub fn new(update: U) -> (Processor<S, U>, Sender<Payload>) {
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
    
    pub fn new_callback(update: U) -> (CallbackSender, Sender<Payload>) {
        let (processor, inbound_tx) = Self::new(update);

        let inbound_tx_clone = inbound_tx.clone();
        let processor = Mutex::new(processor);
        let callback = CallbackSender::callback(move |payload| {
            inbound_tx_clone.send(payload)?;

            processor.lock()
            .map_err(|err| NodeError::Poison(err.to_string()))?
            .process();

            Ok(())
        });

        (callback, inbound_tx)
    }

    pub fn process(&mut self) {
        for payload in self.inbound.try_iter() {
            self.node_tx.send(payload)
            .unwrap_or_else(|err| 
                panic!("Unexpected error while sending message. self.node_tx.send(payload): Err({err})")
            );

            while let Ok(payload) = self.node_rx.try_recv() {
                log::info!("self.node_rx.try_recv() {:?}", payload);
                if !self.handled_messages.contains(payload.key()) {
                    self.handled_messages.insert(payload.key().clone());
    
                    self.node.apply(&payload);
                    
                    let message = S::Message::from_payload(0, payload.clone());
                    (self.update)(&self.node, message, payload);
                }
            }
                
            self.handled_messages.clear();
        }
    }
}
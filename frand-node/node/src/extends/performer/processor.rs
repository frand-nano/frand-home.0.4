use std::{collections::HashSet, sync::{Arc, Mutex}};
use crossbeam::channel::{unbounded, Receiver};
use bases::{CallbackSender, PayloadKey};
use result::NodeError;
use crate::*;

pub struct Processor<N: NodeBase> {     
    node: N,    
    node_rx: Receiver<Payload>, 
    handled_messages: HashSet<PayloadKey>,
}

impl<N: 'static + NodeBase> Processor<N> 
{
    pub fn new_node<U>(update: U) -> N
    where U: 'static + Fn(&N, N::Message, Payload)
    {
        N::new(&Self::new_callback(update), vec![], None)
    }

    pub fn new_node_with<U>(node: &N, update: U) -> N
    where U: 'static + Fn(&N, N::Message, Payload)
    {
        node.set_callback(&Self::new_callback(update));
        node.clone()
    }

    fn new_callback<U>(update: U) -> CallbackSender 
    where U: 'static + Fn(&N, N::Message, Payload)    
    {        
        let (node_tx, node_rx) = unbounded();

        let callback = CallbackSender::Sender(node_tx.clone());
        let processor = Self { 
            node: N::new(&callback, vec![], None), 
            node_rx, 
            handled_messages: HashSet::new(),
        };

        let processor = Mutex::new(processor);
        let callback = CallbackSender::Callback(
            Arc::new(move |payload| {
                processor.lock()
                .map_err(|err| NodeError::Poison(err.to_string()))?
                .process(&update, payload);

                Ok(())
            }
        ));

        callback
    }

    pub fn process<U>(&mut self, update: &U, mut payload: Payload) 
    where U: 'static + Fn(&N, N::Message, Payload)    
    {
        loop {
            if !self.handled_messages.contains(payload.key()) {
                self.handled_messages.insert(payload.key().clone());

                self.node.apply(&payload);
                
                let message = N::Message::from_payload(0, payload.clone());
                (update)(&self.node, message, payload);
            }
            match self.node_rx.try_recv() {
                Ok(recv) => payload = recv,
                Err(_) => break,
            }
        }
            
        self.handled_messages.clear();
    }
}
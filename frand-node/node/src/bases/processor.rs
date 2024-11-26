use std::collections::HashSet;
use crossbeam::channel::{unbounded, Receiver};
use bases::PayloadKey;
use crate::*;

pub struct Processor<N: NodeBase> {     
    node: N,    
    node_rx: Receiver<Payload>, 
    handled_messages: HashSet<PayloadKey>,
}

impl<N: 'static + NodeBase> Processor<N> 
{
    pub fn new(node: &N) -> Self {  
        let (node_tx, node_rx) = unbounded();

        Self { 
            node: node.fork(move |payload| {
                node_tx.send(payload).unwrap()
            }), 
            node_rx, 
            handled_messages: HashSet::new(),
        }
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
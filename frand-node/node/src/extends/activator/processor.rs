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

    pub fn new_callback<U>(update: U) -> CallbackSender 
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
                let mut processor = processor.lock()
                .map_err(|err| NodeError::Poison(err.to_string()))?;

                process(&mut processor, &update, payload);

                Ok(())
            }
        ));

        callback
    }
}

fn process<N: 'static + NodeBase, U>(processor: &mut Processor<N>, update: &U, mut payload: Payload) 
where U: 'static + Fn(&N, N::Message, Payload)    
{
    loop {
        if !processor.handled_messages.contains(payload.key()) {
            processor.handled_messages.insert(payload.key().clone());

            processor.node.apply(&payload);
            
            let message = N::Message::from_payload(0, payload.clone());
            (update)(&processor.node, message, payload);
        }
        match processor.node_rx.try_recv() {
            Ok(recv) => payload = recv,
            Err(_) => break,
        }
    }
        
    processor.handled_messages.clear();
}
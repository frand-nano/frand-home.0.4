use std::collections::HashSet;
use crossbeam::channel::{unbounded, Receiver};
use bases::NodeKey;
use crate::*;

pub struct Processor<N: NodeBase> {     
    node: N,    
    node_rx: Receiver<Packet>, 
    handled_messages: HashSet<NodeKey>,
}

impl<N: 'static + NodeBase> Processor<N> 
{
    pub fn new(node: &N) -> Self {  
        let (node_tx, node_rx) = unbounded();

        Self { 
            node: node.fork(move |packet| {
                node_tx.send(packet).unwrap()
            }), 
            node_rx, 
            handled_messages: HashSet::new(),
        }
    }

    pub fn process<U>(&mut self, update: &U, mut packet: Packet) 
    where U: 'static + Fn(&N, N::Message, Packet)    
    {
        loop {
            if !self.handled_messages.contains(packet.key()) {
                self.handled_messages.insert(packet.key().clone());

                self.node.apply_packet(&packet);
                
                let message = N::Message::from_packet(0, &packet);
                (update)(&self.node, message, packet);
            }
            match self.node_rx.try_recv() {
                Ok(recv) => packet = recv,
                Err(_) => break,
            }
        }
            
        self.handled_messages.clear();
    }
}
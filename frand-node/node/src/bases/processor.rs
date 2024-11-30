use std::collections::HashSet;
use crossbeam::channel::{unbounded, Receiver};
use bases::{NodeKey, Reporter};
use crate::*;

pub struct Processor<N: NodeBase> {     
    node: N,    
    node_rx: Receiver<Packet>, 
    handled_messages: HashSet<NodeKey>,
}

impl<N: NodeBase> Processor<N> 
{
    pub fn new() -> Self {  
        let (node_tx, node_rx) = unbounded();

        let node = N::new(
            vec![], 
            None, 
            Reporter::Sender(node_tx),
        );

        Self { 
            node, 
            node_rx, 
            handled_messages: HashSet::new(),
        }
    }

    pub fn process<F>(&mut self, mut packet: Packet, mut update: F) 
    where F: FnMut(&N, Packet, N::Message) {
        loop {
            if !self.handled_messages.contains(packet.key()) {
                self.handled_messages.insert(packet.key().clone());

                let message = N::Message::from_packet(0, &packet);

                self.node.apply_packet(&packet);

                update(&self.node, packet, message);
            }
            match self.node_rx.try_recv() {
                Ok(recv) => packet = recv,
                Err(_) => break,
            }
        }
            
        self.handled_messages.clear();
    }
}
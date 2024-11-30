use std::collections::HashSet;
use bases::{NodeKey, Reporter};
use futures::future::LocalBoxFuture;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use crate::*;

pub struct AsyncProcessor<N: NodeBase> {     
    node: N,    
    node_rx: UnboundedReceiver<LocalBoxFuture<'static, Packet>>, 
    output_tx: UnboundedSender<Packet>, 
    handled_messages: HashSet<NodeKey>,
}

impl<N: NodeBase> AsyncProcessor<N> 
{
    pub fn new() -> (Self, UnboundedReceiver<Packet>) {  
        let (node_tx, node_rx) = unbounded_channel();
        let (output_tx, output_rx) = unbounded_channel();

        let node = N::new(
            vec![], 
            None, 
            Reporter::FutureSender(node_tx),
        );

        (
            Self { 
                node, 
                node_rx, 
                output_tx,
                handled_messages: HashSet::new(),
            },
            output_rx,
        )
    }

    pub async fn process<F>(&mut self, mut packet: Packet, mut update: F) 
    where F: FnMut(&N, Packet, N::Message) {
        loop {
            if !self.handled_messages.contains(packet.key()) {
                self.handled_messages.insert(packet.key().clone());

                let message = N::Message::from_packet(0, &packet);

                self.node.apply_packet(&packet);
                self.output_tx.send(packet.clone()).unwrap();

                update(&self.node, packet, message);

            }
            match self.node_rx.try_recv() {
                Ok(recv) => packet = recv.await,
                Err(_) => break,
            }
        }
            
        self.handled_messages.clear();
    }
}
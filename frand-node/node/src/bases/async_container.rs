use std::{ops::{Deref, DerefMut}, sync::Arc};
use tokio::{pin, select, sync::mpsc::UnboundedReceiver};
use super::{NodeBase, Packet, AsyncProcessor, Reporter};

pub struct AsyncContainer<N: NodeBase> {
    processor: AsyncProcessor<N>,
    processed_rx: UnboundedReceiver<Packet>, 
    node: N,
}

impl<N: NodeBase> Deref for AsyncContainer<N> {
    type Target = N;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl<N: NodeBase> DerefMut for AsyncContainer<N> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.node }
}

impl<N: NodeBase> AsyncContainer<N> {
    pub fn new<F>(callback: F) -> Self 
    where F: 'static + Fn(Packet) {
        let node = N::new(
            vec![], 
            None, 
            Reporter::Callback(
                Arc::new(move |packet| callback(packet))
            ),
        );

        let (processor, processed_rx) = AsyncProcessor::new();

        Self { 
            processor, 
            processed_rx,
            node, 
        }
    }

    pub async fn process<F>(&mut self, packet: Packet, update: F) 
    where F: FnMut(&N, Packet, N::Message) {
        let future = self.processor.process(packet, update);
        pin!(future);

        loop {
            select! {
                _ = &mut future => { break; },
                Some(packet) = self.processed_rx.recv() => {
                    self.node.apply_packet(&packet);
                },
            }
        }
    }
}
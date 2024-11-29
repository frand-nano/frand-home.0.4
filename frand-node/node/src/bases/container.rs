use std::{ops::{Deref, DerefMut}, sync::Arc};

use crossbeam::channel::Receiver;

use super::{NodeBase, Packet, Processor, Reporter};

pub struct Container<N: NodeBase> {
    processor: Processor<N>,
    output_rx: Receiver<Packet>, 
    node: N,
}

impl<N: NodeBase> Deref for Container<N> {
    type Target = N;
    fn deref(&self) -> &Self::Target { &self.node }
}

impl<N: NodeBase> DerefMut for Container<N> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.node }
}

impl<N: NodeBase> Container<N> {
    pub fn new<F>(callback: F) -> Self 
    where F: 'static + Fn(Packet) {
        let node = N::new(
            vec![], 
            None, 
            Reporter::Callback(
                Arc::new(move |packet| callback(packet))
            ),
        );

        let (processor, output_rx) = Processor::new();

        Self { 
            processor, 
            output_rx,
            node, 
        }
    }

    pub fn process<F>(&mut self, packet: Packet, update: F) 
    where F: FnMut(&N, &Packet, N::Message) {
        self.processor.process(packet, update);
        while let Ok(packet) = self.output_rx.try_recv() {
            self.node.apply_packet(&packet);
        }
    }
}
use std::{ops::{Deref, DerefMut}, sync::Arc};
use super::{NodeBase, Packet, Processor, Reporter};

pub struct Container<N: NodeBase> {
    processor: Processor<N>,
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

        Self { 
            processor: Processor::new(), 
            node, 
        }
    }

    pub fn process<F>(&mut self, packet: Packet, update: F) 
    where F: FnMut(&N, Packet, N::Message) {
        self.processor.process(packet, update);
    }
}
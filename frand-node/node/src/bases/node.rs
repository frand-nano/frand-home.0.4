use std::sync::Arc;
use super::{message::{Packet, NodeId}, state::StateBase, ElementBase};

pub trait NodeBase: ElementBase + Stater<Self::State> {   
    fn new() -> Self { Self::new_child(vec![], None) }

    fn new_child(  
        key: Vec<NodeId>,
        id: Option<NodeId>,
    ) -> Self;

    fn new_activate<F>(callback: F) -> Self
    where F: 'static + Fn(Packet) {
        let result = Self::new();
        result.activate(callback);
        result
    }

    fn emit(&self, state: Self::State);
    fn emit_packet(&self, packet: Packet);

    fn set_callback<F>(&self, callback: &Arc<F>)  
    where F: 'static + Fn(Packet);

    fn activate<F>(&self, callback: F) -> &Self 
    where F: 'static + Fn(Packet);

    fn fork<F>(&self, callback: F) -> Self 
    where F: 'static + Fn(Packet);

    fn inject(&self, process: fn(&Self, &Packet, Self::Message)) -> &Self;

    fn call_process(&self, depth: usize, packet: &Packet);
}

pub trait Stater<S: StateBase> {
    fn apply(&mut self, state: S);
    fn apply_packet(&mut self, packet: &Packet);

    fn apply_packets<I>(&mut self, packets: I) 
    where 
        I: Iterator<Item = Packet>,
        I::Item: AsRef<Packet>,
    {
        for packet in packets {
            self.apply_packet(packet.as_ref());
        }
    }
}

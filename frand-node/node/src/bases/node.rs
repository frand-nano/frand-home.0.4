use std::future::Future;
use super::{message::{NodeId, Packet}, state::StateBase, ElementBase, Reporter};

pub trait NodeBase: ElementBase + Stater<Self::State> {   
    fn new(  
        key: Vec<NodeId>,
        id: Option<NodeId>,
        reporter: Reporter,
    ) -> Self;

    fn emit(&self, state: Self::State);
    fn emit_packet(&self, packet: Packet);
    fn emit_future<Fu>(&self, future: Fu) where Fu: 'static + Future<Output = Self::State>;
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

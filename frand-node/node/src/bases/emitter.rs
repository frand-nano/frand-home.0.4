use std::{fmt::Debug, future::Future, sync::Arc};
use bases::{NodeId, NodeKey};
use crossbeam::channel::Sender;
use futures::{future::LocalBoxFuture, FutureExt};
use tokio::sync::mpsc::UnboundedSender;
use crate::*;

#[derive(Clone)]
pub enum Reporter {
    Callback(Arc<dyn Fn(Packet)>),
    Sender(Sender<Packet>),
    FutureSender(UnboundedSender<LocalBoxFuture<'static, Packet>>),
    None,
}

impl Default for Reporter {
    fn default() -> Self { Reporter::None }
}

impl Debug for Reporter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Callback(_) => f.write_str("Callback(Arc<dyn Fn(Packet)>)"),
            Self::Sender(sender) => f.debug_tuple("Sender").field(sender).finish(),
            Self::FutureSender(sender) => f.debug_tuple("FutureSender").field(sender).finish(),
            Self::None => f.debug_tuple("None").finish(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Emitter<V: StateBase = ()> {
    depth: usize,
    key: NodeKey,
    value: V,
    reporter: Reporter,    
}

impl<V: StateBase> PartialEq for Emitter<V> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<V: StateBase> Clone for Emitter<V> {
    fn clone(&self) -> Self {
        log::debug!("Emitter<N>::clone key:{:?}, value:{:?}", self.key, self.value);
        Self { 
            depth: self.depth.clone(), 
            key: self.key.clone(), 
            value: self.value.clone(),
            reporter: self.reporter.clone(), 
        }
    }
}

impl<V: StateBase> Emitter<V> {
    pub fn depth(&self) -> usize { self.depth }
    pub fn value(&self) -> &V { &self.value }
    pub fn value_mut(&mut self) -> &mut V { &mut self.value }

    pub fn new(
        key: Vec<NodeId>,
        reporter: Reporter,
    ) -> Self {
        log::debug!("Emitter<N>::new {:?}", key);
        Self { 
            depth: key.len(),
            key: key.into_boxed_slice(),   
            value: V::default(),            
            reporter,
        }
    }  

    pub fn emit<S: StateBase>(&self, state: S) {
        self.emit_packet(Packet::new(self.key.clone(), state));
    }

    pub fn emit_packet(&self, packet: Packet) {
        match &self.reporter {
            Reporter::Callback(callback) => callback(packet),
            Reporter::Sender(sender) => sender.send(packet).unwrap(),
            Reporter::FutureSender(sender) => sender.send(async{packet}.boxed()).unwrap(),
            Reporter::None => (),
        }
    }

    pub fn emit_future<S: 'static + StateBase, Fu>(&self, future: Fu) 
    where Fu: 'static + Future<Output = S> {
        match &self.reporter {
            Reporter::Callback(_) => unimplemented!(),
            Reporter::Sender(_) => unimplemented!(),
            Reporter::FutureSender(sender) => {
                let key = self.key.clone();
                let future = future.boxed_local();
                sender.send(
                    async move { 
                        Packet::new(key, future.await) 
                    }
                    .boxed_local()
                ).unwrap()
            },
            Reporter::None => (),
        }
    }
}
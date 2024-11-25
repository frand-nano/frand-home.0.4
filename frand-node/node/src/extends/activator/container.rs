use std::{marker::PhantomData, sync::Arc};
use bases::CallbackSender;
use crate::*;

pub struct Container<N: NodeBase>(PhantomData<N>);

impl<N: 'static + NodeBase> Container<N> 
{
    pub fn new_node<U>(update: U) -> N
    where U: 'static + Fn(Payload)
    {
        N::new(&Self::new_callback(update), vec![], None)
    }

    pub fn new_node_with<U>(node: &N, update: U) -> N
    where U: 'static + Fn(Payload)
    {
        node.set_callback(&Self::new_callback(update));
        node.clone()
    }

    pub fn new_callback<U>(update: U) -> CallbackSender 
    where U: 'static + Fn(Payload)    
    {        
        CallbackSender::Callback(
            Arc::new(move |payload| {
                (update)(payload);
                Ok(())
            }
        ))
    }
}
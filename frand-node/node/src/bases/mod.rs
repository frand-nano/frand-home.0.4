use std::fmt::Debug;

mod emitter;
mod message;
mod state;
mod node;
mod processor;
mod container;
mod async_processor;
mod async_container;

pub use self::{
    emitter::*,
    message::*,
    state::*,
    node::*,
    processor::*,
    container::*,
    async_processor::*,
    async_container::*,
};

pub trait ElementBase: Debug + Clone + Default + Sized + PartialEq {   
    type State: StateBase;
    type Node: NodeBase;
    type Message: MessageBase;
}
use std::fmt::Debug;

mod emitter;
mod processor;
mod message;
mod state;
mod node;
mod container;

pub use self::{
    emitter::*,
    processor::*,
    message::*,
    state::*,
    node::*,
    container::*,
};

pub trait ElementBase: Debug + Clone + Default + Sized + PartialEq {   
    type State: StateBase;
    type Node: NodeBase;
    type Message: MessageBase;
}
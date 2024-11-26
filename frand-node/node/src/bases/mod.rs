use std::fmt::Debug;

mod emitter;
mod processor;
mod message;
mod state;
mod node;

pub use self::{
    emitter::*,
    processor::*,
    message::*,
    state::*,
    node::*,
};

pub trait ElementBase: Debug + Clone + Default + Sized + PartialEq {   
    type State: StateBase;
    type Node: NodeBase;
    type Message: MessageBase;
}
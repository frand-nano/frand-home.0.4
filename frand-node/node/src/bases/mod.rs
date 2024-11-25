use std::fmt::Debug;

mod emitter;
mod message;
mod state;
mod node;

pub use self::{
    emitter::*,
    message::*,
    state::*,
    node::*,
};

pub trait ElementBase: Debug + Clone + Default + Sized + PartialEq {   
    type State: StateBase;
    type Node: NodeBase;
    type Message: MessageBase;
}
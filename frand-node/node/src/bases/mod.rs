use std::fmt::Debug;

mod callback;
mod message;
mod state;
mod node;

pub use self::{
    callback::*,
    message::*,
    state::*,
    node::*,
};

pub trait ElementBase: Debug + Clone + Default + Sized + PartialEq {   
    type State: StateBase;
    type Node: NodeBase;
    type Message: MessageBase;
}
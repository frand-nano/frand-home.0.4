use std::fmt::Debug;
use serde::{de::DeserializeOwned, Serialize};
use super::{message::MessageBase, node::NodeBase};

pub trait StateBase: Default + Debug + Clone + Sized + PartialEq + Serialize + DeserializeOwned {
    type Node: NodeBase<Self>;
    type Message: MessageBase;
}
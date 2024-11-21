use std::fmt::Debug;
use serde::{de::DeserializeOwned, Serialize};
use super::{message::MessageBase, node::NodeBase};

pub trait StateBase: Default + Debug + Clone + PartialEq + Serialize + DeserializeOwned 
where 
<Self as StateBase>::Node: NodeBase<State = Self>,
<Self as StateBase>::Message: MessageBase<State = Self>,
{
    type Node: NodeBase;
    type Message: MessageBase;
}
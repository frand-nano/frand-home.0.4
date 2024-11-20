pub use std::sync::mpsc::Sender;

pub use crate::{
    result::{ComponentError, Result},
    bases::{Callback, MessageBase, MessageData, MessageError, MessageDataId, NodeBase,StateBase},
    extends::Node,
};

pub mod reexport_serde {
    pub use serde::{Serialize, Deserialize};
}
pub use prelude::*;

pub mod result;
pub mod bases;
pub mod extends;

pub mod prelude {
    pub use frand_node_macro::*;

    pub use crate::{
        bases::{
            Packet, Emitter, Container,
            ElementBase, StateBase, NodeBase, MessageBase, Stater,
        },
        extends::Node,
    };
}

pub mod macro_prelude {
    pub use std::{cell::{Ref, RefCell}, ops::{Deref, DerefMut}, borrow::BorrowMut, sync::Arc};
    pub use serde::{Serialize, Deserialize};
    pub use crossbeam::channel::{Sender, Receiver};

    pub use crate::{
        prelude::*,
        result::{Result, NodeError},
        bases::{NodeId, NodeKey, Reporter},
    };
}
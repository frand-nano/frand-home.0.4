pub use prelude::*;

pub mod result;
pub mod bases;
pub mod extends;

pub mod prelude {
    pub use frand_node_macro::*;

    pub use crate::{
        bases::{
            Payload, Emitter,
            ElementBase, StateBase, NodeBase, MessageBase, Stater, 
            Reporter,
        },
        extends::Node,
    };
}

pub mod macro_prelude {
    pub use std::{cell::{Ref, RefCell}, ops::{Deref, DerefMut}, borrow::BorrowMut};
    pub use serde::{Serialize, Deserialize};
    pub use crossbeam::channel::{Sender, Receiver};

    pub use crate::{
        prelude::*,
        result::{Result, NodeError},
        bases::{PayloadId, PayloadKey, Reporter},
    };
}
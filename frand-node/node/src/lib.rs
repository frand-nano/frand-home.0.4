pub use prelude::*;

pub mod result;
pub mod bases;
pub mod extends;

pub mod prelude {
    pub use frand_node_macro::*;

    pub use crate::{
        bases::{
            Payload,
            StateBase, NodeBase, MessageBase, Emitter, Stater,
        },
        extends::{Node, Performer, Container},
    };
}

pub mod macro_prelude {
    pub use std::cell::RefCell;
    pub use std::ops::{Deref, DerefMut};
    pub use serde::{Serialize, Deserialize};
    pub use crossbeam::channel::{Sender, Receiver};

    pub use crate::{
        prelude::*,
        result::{Result, NodeError},
        bases::{PayloadId, PayloadKey, CallbackSender},
    };
}
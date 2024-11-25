pub use prelude::*;

pub mod result;
pub mod bases;
pub mod extends;

pub mod prelude {
    pub use frand_node_macro::*;

    pub use crate::{
        bases::{
            Payload,
            ElementBase, StateBase, NodeBase, MessageBase, Emitter, Stater, 
        },
        extends::{Node, Container, Processor},
    };
}

pub mod macro_prelude {
    pub use std::cell::{Ref, RefCell};
    pub use std::ops::{Deref, DerefMut};
    pub use serde::{Serialize, Deserialize};
    pub use crossbeam::channel::{Sender, Receiver};

    pub use crate::{
        prelude::*,
        result::{Result, NodeError},
        bases::{PayloadId, PayloadKey, CallbackSender},
    };
}
pub use prelude::*;

pub mod result;
pub mod bases;
pub mod extends;

pub mod prelude {
    pub use frand_node_macro::*;

    pub use crate::{
        bases::{
            MessageData,
            StateBase, NodeBase, MessageBase, ContainerBase, Emitter, Stater,
        },
        extends::{Node, Performer, Container},
    };
}

pub mod macro_prelude {
    pub use std::ops::{Deref, DerefMut};
    pub use serde::{Serialize, Deserialize};
    pub use crossbeam::channel::{Sender, Receiver};

    pub use crate::{
        prelude::*,
        result::{NodeError, Result},
        bases::{
            Callback, MessageError,
            MessageDataId, CallbackSender,
        },
    };
}
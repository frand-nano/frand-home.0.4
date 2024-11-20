use std::fmt::Debug;
use serde::{de::DeserializeOwned, Serialize};

use super::{message::MessageBase, node::NodeBase};

mod frand_node {
    pub mod macro_prelude {
        pub use crate::macro_prelude::*;
    }
}

pub trait StateBase: Default + Debug + Clone + PartialEq + Serialize + DeserializeOwned 
where 
<Self as StateBase>::Node: NodeBase<State = Self>,
<Self as StateBase>::Message: MessageBase<State = Self>,
{
    type Node: NodeBase;
    type Message: MessageBase;
}

#[macro_export]
macro_rules! impl_state_for {
    ( $head: ty $(,$tys: ty)* $(,)? ) => { 
        impl_state_for!{ @inner($head, $($tys,)*) }
    };
    ( @inner($($tys: ty,)*) ) => {    
        $(
            impl frand_node::macro_prelude::StateBase for $tys {
                type Node = frand_node::macro_prelude::Node<Self>;
                type Message = Self;
            }

            impl frand_node::macro_prelude::MessageBase for $tys {
                type State = Self;

                fn deserialize(
                    data: frand_node::macro_prelude::MessageData,
                ) -> frand_node::macro_prelude::Result<Self> {
                    match data.next() {
                        (Some(0), data) => Ok(data.deserialize()?),
                        (Some(_), data) => Err(data.error(
                            format!("S::deserialize() unknown id"),
                        )),
                        (None, data) => Err(data.error(
                            format!("S::deserialize() data has no more id"),
                        )),
                    }     
                }
            }
        )*      
    };
}

impl_state_for!{ 
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64,
    char, bool, (),
    String, 
}
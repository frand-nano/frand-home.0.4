use std::fmt::Debug;
use serde::{de::DeserializeOwned, Serialize};

use super::node::NodeBase;

mod frand_node {
    pub use crate::*;
}

pub trait StateBase: Default + Debug + Clone + PartialEq + Serialize + DeserializeOwned {
    type Node: NodeBase<Self>;
}

#[macro_export]
macro_rules! impl_state_for {
    ( $head: ty $(,$tys: ty)* $(,)? ) => { 
        impl_state_for!{ @inner($head, $($tys,)*) }
    };
    ( @inner($($tys: ty,)*) ) => {    
        $(
            impl frand_node::StateBase for $tys {
                type Node = frand_node::Node<Self>;
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
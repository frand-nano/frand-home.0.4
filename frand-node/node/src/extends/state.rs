mod frand_node {
    pub mod __macro_prelude {
        pub use crate::__macro_prelude::*;
    }
}

#[macro_export]
macro_rules! impl_state_for {
    ( $head: ty $(,$tys: ty)* $(,)? ) => { 
        impl_state_for!{ @inner($head, $($tys,)*) }
    };
    ( @inner($($tys: ty,)*) ) => {    
        $(
            impl frand_node::__macro_prelude::StateBase for $tys {
                type Node = frand_node::__macro_prelude::Node<Self>;
                type Message = Self;
            }

            impl frand_node::__macro_prelude::MessageBase for $tys {
                type State = Self;

                fn deserialize(
                    data: frand_node::__macro_prelude::MessageData,
                ) -> frand_node::__macro_prelude::Result<Self> {
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
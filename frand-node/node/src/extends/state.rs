mod frand_node {
    pub mod macro_prelude {
        pub use crate::macro_prelude::*;
    }
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
                    depth: usize,
                    data: frand_node::macro_prelude::MessageData,
                ) -> Self {
                    match data.get_id(depth) {
                        Some(0) => data.read_state(),
                        Some(_) => Err(data.error(depth, "unknown id")),
                        None => Err(data.error(depth, "data has no more id")),
                    }     
                    .unwrap_or_else(|err| panic!("{}::deserialize() Err({err})", stringify!($tys)))
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
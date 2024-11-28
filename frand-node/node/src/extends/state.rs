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
            impl frand_node::macro_prelude::ElementBase for $tys {
                type State = Self;
                type Node = frand_node::macro_prelude::Node<Self>;
                type Message = Self;
            }

            impl frand_node::macro_prelude::StateBase for $tys {

            }

            impl frand_node::macro_prelude::MessageBase for $tys {
                fn from_packet(
                    depth: usize,
                    packet: &frand_node::macro_prelude::Packet,
                ) -> Self {
                    match packet.get_id(depth) {
                        Some(_) => Err(packet.error(depth, "unknown id")),
                        None => Ok(packet.read_state()),
                    }     
                    .unwrap_or_else(|err| panic!("{}::from_packet() Err({err})", stringify!($tys)))
                }
            }
        )*      
    };
}

impl_state_for!{ 
    i8, i16, i32, i64, i128, 
    u8, u16, u32, u64, u128, 
    f32, f64,
    char, bool, (),
}
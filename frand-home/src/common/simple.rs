use frand_node::*;

#[node_macro(
    node_attrs(#[derive(yew::Properties)])
)]
pub struct Simple {
    pub sub1: SimpleSub,
    pub sub2: SimpleSub,
    pub message_count: u32,
}

simple_macro!{}

#[node_macro(
    node_attrs(#[derive(yew::Properties)])
)]
pub struct SimpleSub {
    pub number1: i32,
    pub number2: i32,
    pub number3: i32,
}
 
simple_sub_macro!{}
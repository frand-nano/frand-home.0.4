use frand_node::*;

#[node]
#[derive(yew::Properties)]
pub struct Shared {
    pub number1: i32,
    pub number2: i32,
    pub number3: i32,
    pub number4: i32,
}
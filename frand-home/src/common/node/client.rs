use frand_node::*;

#[node(node_attrs(#[derive(yew::Properties)]))]
pub struct Client {
    number1: i32,
    number2: i32,
    number3: i32,
}
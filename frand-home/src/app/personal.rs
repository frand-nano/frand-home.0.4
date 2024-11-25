use frand_node::*;

#[node(node_attrs(#[derive(yew::Properties)]))]
pub struct Personal {
    pub number1: i32,
    pub number2: i32,
    pub number3: i32,
    pub number4: i32,
}
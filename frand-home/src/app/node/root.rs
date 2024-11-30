use yew::*;
use frand_node::*;
use super::{personal::Personal, shared::Shared};

#[node]
#[derive(Properties)]
pub struct Root {
    shared: Shared,
    personal: Personal,
}

use yew::*;
use frand_node::*;
use crate::app::{node::number_sum::{NumberSum, NumberSumView}, view::NumberSumIncView};

#[node]
#[derive(Properties)]
pub struct Shared {
    pub sum1: NumberSum,
    pub sum2: NumberSum,
    pub sum3: NumberSum,
}

#[function_component]
pub fn SharedView(node: &Shared) -> Html {
    html! {
        <div>
            {"Shared"}
            <NumberSumIncView ..node.sum1.clone() />
            <NumberSumIncView ..node.sum2.clone() />
            <NumberSumView ..node.sum3.clone() />
        </div>
    }
}
use yew::*;
use crate::app::{node::number_sum::{NumberSum, NumberSumView}, view::{NumberInc, NumberIncView}};

#[function_component]
pub fn NumberSumIncView(node: &NumberSum) -> Html {
    html! {
        <div>
            <NumberSumView ..node.clone() />
            <NumberIncView ..NumberInc{ name:"a", number: node.a.clone() } />
            <NumberIncView ..NumberInc{ name:"b", number: node.b.clone() } />
        </div>
    }
}
use yew::*;
use frand_node::*;
use crate::app::view::NumberSumIncView;
use super::number_sum::NumberSum;

#[node]
#[derive(Properties)]
pub struct Personal {
    pub sum1: NumberSum,
    pub sum2: NumberSum,
    pub sum3: NumberSum,
}

#[function_component]
pub fn PersonalView(node: &Personal) -> Html {
    html! {
        <div>
            {"Personal"}
            <NumberSumIncView ..node.sum1.clone() />
            <NumberSumIncView ..node.sum2.clone() />
            <NumberSumIncView ..node.sum3.clone() />
        </div>
    }
}
use yew::*;
use frand_node::*;
use crate::app::view::{IncButton, IncButtonView};

#[node]
#[derive(yew::Properties)]
pub struct Shared {
    pub number1: i32,
    pub number2: i32,
    pub number3: i32,
    pub number4: i32,
}

#[function_component]
pub fn SharedView(shared: &Shared) -> Html {
    log::debug!("Shared::view");   
    html! {
        <div>
            <IncButtonView ..IncButton { number: shared.number1.clone() } />
            <IncButtonView ..IncButton { number: shared.number2.clone() } />
            <IncButtonView ..IncButton { number: shared.number3.clone() } />
            <IncButtonView ..IncButton { number: shared.number4.clone() } />
        </div>
    }
}
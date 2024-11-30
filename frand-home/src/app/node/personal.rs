use yew::*;
use frand_node::*;
use crate::app::view::{IncButton, IncButtonView};

#[node_macro]
#[derive(Properties)]
pub struct Personal {
    pub number1: i32,
    pub number2: i32,
    pub number3: i32,
    pub number4: i32,
}

personal_macro!{}

#[function_component]
pub fn PersonalView(personal: &Personal) -> Html {
    log::debug!("Personal::view");   
    html! {
        <div>
            <IncButtonView ..IncButton { number: personal.number1.clone() } />
            <IncButtonView ..IncButton { number: personal.number2.clone() } />
            <IncButtonView ..IncButton { number: personal.number3.clone() } />
            <IncButtonView ..IncButton { number: personal.number4.clone() } />
        </div>
    }
}
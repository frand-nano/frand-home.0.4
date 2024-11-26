use yew::*;
use frand_node::*;

#[derive(Properties, Clone, PartialEq)]
pub struct IncButton {
    pub number: Node<i32>,
}

#[function_component]
pub fn IncButtonView(button: &IncButton) -> Html {
    log::debug!("IncButton::view");
    let number = button.number.clone();
    let number_value = *button.number;
    let inc = move |_| {
        number.emit(number_value + 1)
    };

    html! {
        <>
        <button onclick = {inc}>
            { format!("number : {number_value} + 1, ") }
        </button>
        </>
    }
}
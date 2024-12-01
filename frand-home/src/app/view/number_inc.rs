use frand_node::*;
use yew::*;

#[derive(Properties, Clone, PartialEq)]
pub struct NumberInc {
    pub name: &'static str,
    pub number: Node<i32>,
}

#[function_component]
pub fn NumberIncView(node: &NumberInc) -> Html {
    let name = node.name;
    let number = node.number.clone();
    let number_value = *node.number;
    let inc = move |_| {
        number.emit(number_value + 1)
    };

    html! {
        <button onclick = {inc}>
            { format!("inc {name}: {number_value}") }
        </button>
    }
}
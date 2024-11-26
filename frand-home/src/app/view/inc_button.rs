use yew::*;
use frand_node::*;

#[derive(Properties, Clone, PartialEq)]
pub struct IncButton {
    pub number: Node<i32>,
}

impl From<Node<i32>> for IncButton {
    fn from(value: Node<i32>) -> Self { Self { number: value } }
}

impl Component for IncButton {
    type Message = ();
    type Properties = Self;

    fn create(context: &Context<Self>) -> Self {
        log::debug!("IncButton::create");
        context.props().clone()
    }

    fn view(&self, _: &Context<Self>) -> Html {
        log::debug!("IncButton::view");
        let number = self.number.clone();
        let number_value = *self.number;
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
}
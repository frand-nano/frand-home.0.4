use frand_node::*;
use yew::{function_component, Html};

#[node]
pub struct Test {
    pub number: i32,
}

test_macro!{}

#[allow(unused)]
pub fn render() {
    yew::Renderer::<App>::new().render();    
}

#[function_component]
fn App() -> Html {
    use yew::prelude::*;

    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
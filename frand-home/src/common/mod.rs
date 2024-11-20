use frand_node::*;
use yew::{function_component, Html};

#[node]
pub struct Test {
    pub number1: i32,
    pub number2: i32,
    pub number3: i32,
}

test_macro!{}

/*
#[test]
fn test() -> anyhow::Result<()> {
    let mut component = Component::<Test::State>::new();

    component.node().number1.emit(&1)?;

    for i in 0..10 {
        component.perform()?;
        assert!(*component.node().number1.value() == i * 3 + 1);
        assert!(*component.node().number2.value() == i * 3 + 2);
        assert!(*component.node().number3.value() == i * 3 + 3);
    }    

    Ok(())
}
*/

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
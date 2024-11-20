
use frand_node::*;
use yew::{function_component, Html};

#[frand_node::node_macro]
pub struct Test {
    pub sub1: TestSub,
    pub sub2: TestSub,
}

test_macro!{}

#[frand_node::node_macro]
pub struct TestSub {
    pub number1: i32,
    pub number2: i32,
    pub number3: i32,
}

test_sub_macro!{}

#[frand_node::component_macro(Test)]
pub struct TestComponent {

}

test_component_macro!{}

impl Component for TestComponent {
    fn update(node: &Self::Node, message: Self::Message) -> anyhow::Result<()> {
        match message {
            TestMessage::Sub1(
                TestSubMessage::Number1(number1)
            ) => node.sub1.number2.emit(&(number1 + 1))?,

            TestMessage::Sub1(
                TestSubMessage::Number2(number2)
            ) => node.sub1.number3.emit(&(number2 + 1))?,

            TestMessage::Sub1(
                TestSubMessage::Number3(number3)
            ) => node.sub1.number1.emit(&(number3 + 1))?,

            _ => {},
        }
        Ok(())
    }
}

#[test]
fn test() -> anyhow::Result<()> {
    let mut component = TestComponent::new();

    component.node().sub1.number1.emit(&1)?;

    for i in 0..10 {
        component.perform()?;

        assert!(*component.node().sub1.number1.value() == i * 3 + 1);
        assert!(*component.node().sub1.number2.value() == i * 3 + 2);
        assert!(*component.node().sub1.number3.value() == i * 3 + 3);
    }    

    Ok(())
}

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
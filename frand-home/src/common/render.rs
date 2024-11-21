
use frand_node::*;
use yew::{html, Html, Properties};

#[node_macro(
    node_attrs(#[derive(Properties)])
)]
pub struct Test {
    pub sub1: TestSub,
    pub sub2: TestSub,
}

test_macro!{}

#[node_macro(
    node_attrs(#[derive(Properties)])
)]
pub struct TestSub {
    pub number1: i32,
    pub number2: i32,
    pub number3: i32,
}

test_sub_macro!{}

#[component_macro(
    state(Test)
)]
pub struct TestComponent {

}

test_component_macro!{}

impl Component for TestComponent {
    fn update(node: &Self::Node, message: Self::Message) -> anyhow::Result<()> {
        use TestMessage::*;
        use TestSubMessage::*;

        match message {
            sub1(number1(n)) => node.sub1.number2.emit(&(n + 1))?,
            sub1(number2(n)) => node.sub1.number3.emit(&(n + 1))?,

            sub2(number1(n)) => node.sub2.number2.emit(&(n * 2))?,
            sub2(number2(n)) => node.sub2.number3.emit(&(n * 2))?,

            _ => {},
        }
        
        Ok(())
    }
}

#[test]
fn test() -> anyhow::Result<()> {
    let mut component = TestComponent::new();

    component.node().sub1.number1.emit(&0)?;
    component.node().sub2.number1.emit(&4)?;
    component.apply_output()?;

    assert_eq!(*component.node().sub1.number1.value(), 0);
    assert_eq!(*component.node().sub1.number2.value(), 1);
    assert_eq!(*component.node().sub1.number3.value(), 2);

    assert_eq!(*component.node().sub2.number1.value(), 4);
    assert_eq!(*component.node().sub2.number2.value(), 8);
    assert_eq!(*component.node().sub2.number3.value(), 16);

    Ok(())
}

impl yew::Component for TestComponent {
    type Message = TestMod::Message;
    type Properties = TestMod::Node;

    fn create(ctx: &yew::Context<Self>) -> Self {
        log::info!("create");
        
        let mut result = Self::new();
        result.replace_node(ctx.props());
        result
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {    
        log::info!("view");

        let onclick_sub1_number1 = {    
            let num = self.node().sub1.number1.clone();
            move |_| num.emit(&(num.value() + 1)).unwrap()
        };

        let onclick_sub2_number1 = {    
            let num = self.node().sub2.number1.clone();
            move |_| num.emit(&(num.value() + 1)).unwrap()
        };

        html! {
            <div>
            <button onclick = {onclick_sub1_number1}>{ "sub1.number1 +1" }</button>
            <button onclick = {onclick_sub2_number1}>{ "sub2.number1 +1" }</button>
                <p> {"sub1.number1 : "} { self.node().sub1.number1.value() }</p>
                <p> {"sub1.number2 : "} { self.node().sub1.number2.value() }</p>
                <p> {"sub1.number3 : "} { self.node().sub1.number3.value() }</p>
                <p> {"sub2.number1 : "} { self.node().sub2.number1.value() }</p>
                <p> {"sub2.number2 : "} { self.node().sub2.number2.value() }</p>
                <p> {"sub2.number3 : "} { self.node().sub2.number3.value() }</p>
            </div>
        }
    }
}

#[allow(unused)]
pub fn render() {
    let component = TestComponent::new();
    yew::Renderer::<TestComponent>::with_props(component.node().clone()).render();    
}
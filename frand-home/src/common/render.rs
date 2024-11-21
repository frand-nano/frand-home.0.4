
use std::ops::{Deref, DerefMut};

use bases::MessageData;
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

pub struct TestComponent {
    performer: Performer<Test>,
    message_count: usize,
}

impl Deref for TestComponent {
    type Target = Performer<Test>;
    fn deref(&self) -> &Self::Target { &self.performer }
}

impl DerefMut for TestComponent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.performer }
}

impl TestComponent {
    pub fn new(context: &yew::Context<Self>) -> Self {
        let callback = context.link().callback(|message: MessageData| message);

        let performer = Performer::<Test>::new(move |node, message, data| {
            use TestMessage::*;
            use TestSubMessage::*;

            callback.emit(data);

            match message {
                sub1(number1(n)) => {
                    node.sub1.number2.emit(&(n + 1))?;
                },
                sub1(number2(n)) => {
                    node.sub1.number3.emit(&(n + 1))?;
                },
                sub1(number3(n)) => {
                    node.sub1.number1.emit(&(n + 1))?;
                },

                sub2(number1(n)) => {
                    node.sub2.number2.emit(&(n * 2))?;
                    node.sub2.number3.emit(&(n / 2))?;
                },
                sub2(number2(n)) => {
                    node.sub2.number3.emit(&(n * 2))?;
                    node.sub2.number1.emit(&(n / 2))?;
                },
                sub2(number3(n)) => {
                    node.sub2.number1.emit(&(n * 2))?;
                    node.sub2.number2.emit(&(n / 2))?;
                },

                _ => {},
            }
            
            Ok(())
        });

        Self {
            performer,
            message_count: 0,
        }        
    }
}

impl yew::Component for TestComponent {
    type Message = MessageData;
    type Properties = TestMod::Node;

    fn create(context: &yew::Context<Self>) -> Self {
        log::info!("create");
        
        let mut result = Self::new(context);
        result.replace_node(context.props());
        result
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {    
        log::info!("view");

        let add1 = |node: Node<i32>| {
            (
                *node.value(),
                move |_| node.emit(&(node.value() + 1)).unwrap(), 
            )
        };

        let s1n1 = (add1)(self.node().sub1.number1.clone());
        let s1n2 = (add1)(self.node().sub1.number2.clone());
        let s1n3 = (add1)(self.node().sub1.number3.clone());

        let s2n1 = (add1)(self.node().sub2.number1.clone());
        let s2n2 = (add1)(self.node().sub2.number2.clone());
        let s2n3 = (add1)(self.node().sub2.number3.clone());

        html! {
            <div>
                <div>
                    <button onclick = {s1n1.1}>
                        { format!("{} : {} + 1", stringify!(s1n1), s1n1.0) }
                    </button>
                    <button onclick = {s1n2.1}>
                        { format!("{} : {} + 1", stringify!(s1n2), s1n2.0) }
                    </button>
                    <button onclick = {s1n3.1}>
                        { format!("{} : {} + 1", stringify!(s1n3), s1n3.0) }
                    </button>
                </div>
                <div>
                    <button onclick = {s2n1.1}>
                        { format!("{} : {} + 1", stringify!(s2n1), s2n1.0) }
                    </button>
                    <button onclick = {s2n2.1}>
                        { format!("{} : {} + 1", stringify!(s2n2), s2n2.0) }
                    </button>
                    <button onclick = {s2n3.1}>
                        { format!("{} : {} + 1", stringify!(s2n3), s2n3.0) }
                    </button>
                </div>
                <p> {"message_count : "} { self.message_count }</p>
            </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, message: Self::Message) -> bool {
        self.apply(message).unwrap();
        self.message_count += 1;
        true
    }
}

#[allow(unused)]
pub fn render() {
    yew::Renderer::<TestComponent>::new().render();    
}
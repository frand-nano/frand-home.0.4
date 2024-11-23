use std::ops::{Deref, DerefMut};
use frand_node::*;
use yew::{html, Html};
use super::{client_socket::{ClientSocket, SocketMessage}, simple::{Simple, SimpleMessage::*, SimpleMod, SimpleSubMessage::*}};

pub struct SimpleComponent {
    performer: Performer<Simple>,
    socket: ClientSocket<Self>,
    message_count: usize,
}

impl Deref for SimpleComponent {
    type Target = Performer<Simple>;
    fn deref(&self) -> &Self::Target { &self.performer }
}

impl DerefMut for SimpleComponent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.performer }
}

impl SimpleComponent {
    pub fn new(context: &yew::Context<Self>) -> Self {
        let callback = context.link().callback(
            |message: MessageData| SocketMessage::Receive(message)
        );

        let update = move |node: &SimpleMod::Node, message, data| {
            callback.emit(data);

            match message {
                sub1(number1(n)) => {
                    node.sub1.number2.emit(&(n + 1));
                },
                sub1(number2(n)) => {
                    node.sub1.number3.emit(&(n + 1));
                },
                sub1(number3(n)) => {
                    node.sub1.number1.emit(&(n + 1));
                },

                sub2(number1(n)) => {
                    node.sub2.number2.emit(&(n * 2));
                    node.sub2.number3.emit(&(n / 2));
                },
                sub2(number2(n)) => {
                    node.sub2.number3.emit(&(n * 2));
                    node.sub2.number1.emit(&(n / 2));
                },
                sub2(number3(n)) => {
                    node.sub2.number1.emit(&(n * 2));
                    node.sub2.number2.emit(&(n / 2));
                },

                _ => {},
            }
        };

        Self {
            performer: Performer::<Simple>::new_with(context.props(), update),
            socket: ClientSocket::new(context),
            message_count: 0,
        }        
    }
}

impl yew::Component for SimpleComponent {
    type Message = SocketMessage<MessageData>;
    type Properties = SimpleMod::Node;

    fn create(context: &yew::Context<Self>) -> Self {
        Self::new(context)
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {    
        let add1 = |node: Node<i32>| {
            (
                *node.value(),
                move |_| node.emit(&(node.value() + 1)), 
            )
        };

        let s1n1 = (add1)(self.sub1.number1.clone());
        let s1n2 = (add1)(self.sub1.number2.clone());
        let s1n3 = (add1)(self.sub1.number3.clone());

        let s2n1 = (add1)(self.sub2.number1.clone());
        let s2n2 = (add1)(self.sub2.number2.clone());
        let s2n3 = (add1)(self.sub2.number3.clone());

        html! {
            <div>
                <div>
                    <button onclick = {s1n1.1}>
                        { format!("s1n1 : {} + 1", s1n1.0) }
                    </button>
                    <button onclick = {s1n2.1}>
                        { format!("s1n2 : {} + 1", s1n2.0) }
                    </button>
                    <button onclick = {s1n3.1}>
                        { format!("s1n3 : {} + 1", s1n3.0) }
                    </button>
                </div>
                <div>
                    <button onclick = {s2n1.1}>
                        { format!("s2n1 : {} + 1", s2n1.0) }
                    </button>
                    <button onclick = {s2n2.1}>
                        { format!("s2n2 : {} + 1", s2n2.0) }
                    </button>
                    <button onclick = {s2n3.1}>
                        { format!("s2n3 : {} + 1", s2n3.0) }
                    </button>
                </div>
                <p> {"message_count : "} { self.message_count }</p>
            </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, message: Self::Message) -> bool {
        match message {
            SocketMessage::Send(message) => {
                self.socket.send(message);
            },
            SocketMessage::Receive(message) => {
                self.apply(&message);
            },
        }
        
        self.message_count += 1;
        true
    }
}

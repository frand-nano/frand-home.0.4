use extends::Processor;
use frand_node::*;
use frand_web::yew::client_socket::{ClientSocket, SocketMessage};
use yew::{html, Html};
use super::simple::Simple;

pub struct SimpleComponent {
    simple: Simple,
    socket: ClientSocket,
}

impl SimpleComponent {
    pub fn new(context: &yew::Context<Self>) -> Self {
        let callback = context.link().callback(
            |message: Payload| SocketMessage::ToServer(message)
        );

        let update = move |_: &Simple, _, payload| {
            callback.emit(payload);
        };

        Self {
            simple: Processor::<Simple>::new_node_with(context.props(), update),
            socket: ClientSocket::new(context),
        }        
    }
}

impl yew::Component for SimpleComponent {
    type Message = SocketMessage;
    type Properties = Simple;

    fn create(context: &yew::Context<Self>) -> Self {
        Self::new(context)
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {    
        let simple = &self.simple;

        let add1 = |node: Node<i32>| {
            (
                *node.value(),
                move |_| node.emit(node.value() + 1), 
            )
        };

        let s1n1 = (add1)(simple.sub1.number1.clone());
        let s1n2 = (add1)(simple.sub1.number2.clone());
        let s1n3 = (add1)(simple.sub1.number3.clone());

        let s2n1 = (add1)(simple.sub2.number1.clone());
        let s2n2 = (add1)(simple.sub2.number2.clone());
        let s2n3 = (add1)(simple.sub2.number3.clone());

        let message_count = simple.message_count.value();

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
                <p> {"message_count: "} {message_count} </p>
            </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, message: Self::Message) -> bool {
        match message {
            SocketMessage::ToServer(message) => {
                self.socket.send(message);
            },
            SocketMessage::FromServer(message) => {
                self.simple.apply(&message);
            },
        }
        true
    }
}

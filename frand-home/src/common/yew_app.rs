use frand_node::*;
use frand_web::yew::client_socket::{ClientSocket, SocketMessage};
use yew::{html, Html};

use super::node::{client::Client, server::Server};

pub struct YewApp {
    node: YewNode,
    socket: ClientSocket,
}

#[node(node_attrs(#[derive(yew::Properties)]))]
pub struct YewNode {
    server: Server,
    client: Client,
}

impl YewApp {
    pub fn new(context: &yew::Context<Self>) -> Self {
        let callback = context.link().callback(
            |message: Payload| SocketMessage::ToServer(message)
        );

        Self {
            node: Container::<YewNode>::new_node_with(
                context.props(), 
                move |payload| callback.emit(payload),
            ),
            socket: ClientSocket::new(context),
        }        
    }
}

impl yew::Component for YewApp {
    type Message = SocketMessage;
    type Properties = YewNode;

    fn create(context: &yew::Context<Self>) -> Self {
        Self::new(context)
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {    
        let node = &self.node;

        let add1 = |node: Node<i32>| {
            (
                *node.value(),
                move |_| node.emit(node.value() + 1), 
            )
        };

        let sn1 = (add1)(node.server.number1.clone());
        let sn2 = (add1)(node.server.number2.clone());
        let sn3 = (add1)(node.server.number3.clone());

        let cn1 = (add1)(node.client.number1.clone());
        let cn2 = (add1)(node.client.number2.clone());
        let cn3 = (add1)(node.client.number3.clone());

        html! {
            <div>
                <div>
                    <button onclick = {sn1.1}>
                        { format!("sn1 : {} + 1", sn1.0) }
                    </button>
                    <button onclick = {sn2.1}>
                        { format!("sn2 : {} + 1", sn2.0) }
                    </button>
                    <button onclick = {sn3.1}>
                        { format!("sn2 : {} + 1", sn3.0) }
                    </button>
                </div>
                <div>
                    <button onclick = {cn1.1}>
                        { format!("cn1 : {} + 1", cn1.0) }
                    </button>
                    <button onclick = {cn2.1}>
                        { format!("cn2 : {} + 1", cn2.0) }
                    </button>
                    <button onclick = {cn3.1}>
                        { format!("cn3 : {} + 1", cn3.0) }
                    </button>
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, message: Self::Message) -> bool {
        match message {
            SocketMessage::ToServer(payload) => {
                self.socket.send(payload);
            },
            SocketMessage::FromServer(payload) => {
                self.node.apply(&payload);
            },
        }
        true
    }
}

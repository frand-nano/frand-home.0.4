use frand_node::*;
use frand_web::yew::client_socket::{ClientSocket, SocketMessage};
use yew::Html;
use crate::app::root::{view, Root};

pub struct YewApp {
    root: Root,
    socket: ClientSocket,
}

impl YewApp {
    pub fn new(context: &yew::Context<Self>) -> Self {
        let callback = context.link().callback(
            |message: Payload| SocketMessage::ToServer(message)
        );

        Self {
            root: Container::<Root>::new_node_with(
                context.props(), 
                move |payload| callback.emit(payload),
            ),
            socket: ClientSocket::new(context),
        }        
    }
}

impl yew::Component for YewApp {
    type Message = SocketMessage;
    type Properties = Root;

    fn create(context: &yew::Context<Self>) -> Self {
        Self::new(context)
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {    
        view(&self.root)
    }

    fn update(&mut self, _ctx: &yew::Context<Self>, message: Self::Message) -> bool {
        match message {
            SocketMessage::ToServer(payload) => {
                self.socket.send(payload);
            },
            SocketMessage::FromServer(payload) => {
                self.root.apply(&payload);
            },
        }
        true
    }
}

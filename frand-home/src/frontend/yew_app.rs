use frand_node::*;
use frand_web::yew::client_socket::{ClientSocket, SocketMessage};
use yew::{html, Html};
use crate::app::root::{Root, RootView};

pub struct YewApp {
    root: Root,
    socket: ClientSocket,
}

impl YewApp {
    pub fn new(context: &yew::Context<Self>) -> Self {
        let callback = context.link().callback(
            |payload| SocketMessage::ToServer(payload)
        );

        Self {
            root: context.props().set_callback(move |payload| callback.emit(payload)).clone(), 
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
        html! {
            <RootView ..self.root.clone() />
        }
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

use yew::*;
use frand_node::*;
use frand_web::yew::client_socket::{ClientSocket, FromServerSocket};
use crate::app::{personal::PersonalView, shared::SharedView};
use super::{personal::Personal, shared::Shared};

#[node]
#[derive(Properties)]
pub struct Root {
    shared: Shared,
    personal: Personal,
}

pub struct YewApp {
    container: Container<Root>,
    socket: ClientSocket,
}

pub enum Message {
    FromServer(Packet),
    FromNode(Packet),
}

impl From<FromServerSocket> for Message {
    fn from(value: FromServerSocket) -> Self { Self::FromServer(value.into()) }
}

impl yew::Component for YewApp {
    type Message = Message;
    type Properties = Root;

    fn create(context: &Context<Self>) -> Self {
        log::debug!("App::create");
        let socket = ClientSocket::new(context);

        let callback = context.link().callback(
            move |packet| Message::FromNode(packet)
        );

        Self { 
            container: Container::new(move |packet| callback.emit(packet)),
            socket,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {    
        log::debug!("App::view");
        html! {
            <div>
                <SharedView ..self.container.shared.clone() />
                <PersonalView ..self.container.personal.clone() />                
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Message::FromServer(packet) => {
                log::debug!("FromServer({:?})", packet);
                self.container.apply_packet(&packet);
                true
            },
            Message::FromNode(packet) => {
                log::debug!("FromNode");
                self.container.process(packet, |_,packet,_| self.socket.send(packet));
                false
            },
        }
    }
}

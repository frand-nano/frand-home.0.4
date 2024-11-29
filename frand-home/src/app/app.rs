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

/*
#[cfg(not(target_arch = "wasm32"))]
pub mod backend {
    use frand_node::*;
    use uuid::Uuid;
    use tokio::sync::mpsc::UnboundedSender;
    use crate::app::{personal::PersonalMessage, shared::SharedMessage};
    use super::{App, AppMessage, AppMod};

    pub fn handle(
        id: &Uuid,
        send_tx: &UnboundedSender<(Uuid, Packet)>,
        broadcast_tx: &UnboundedSender<Packet>,
        node: &App, 
        message: AppMod::Message, 
        packet: Packet,
    ) {
        use AppMessage::*;
        match message {
            shared(message) => {
                use SharedMessage::*;
                broadcast_tx.send(packet).unwrap();
                match message {
                    number1(n) => node.shared.number2.emit(n + 1),
                    number2(n) => node.shared.number3.emit(n + 1),
                    number3(n) => node.shared.number4.emit(n + 1),
                    number4(n) => node.shared.number1.emit(n + 1),
                    _ => {},
                }
            }
            personal(message) => {
                use PersonalMessage::*;
                send_tx.send((id.clone(), packet)).unwrap();
                match message {
                    number1(n) => {
                        node.personal.number4.emit(n - 1);
                        node.personal.number2.emit(n + 1);
                    },
                    number2(n) => {
                        node.personal.number1.emit(n - 1);
                        node.personal.number3.emit(n + 1);
                    },
                    number3(n) => {
                        node.personal.number2.emit(n - 1);
                        node.personal.number4.emit(n + 1);
                    },
                    number4(n) => {
                        node.personal.number3.emit(n - 1);
                        node.personal.number1.emit(n + 1);
                    },
                    _ => {},
                }
            }
            _ => {},
        }
    }
}
*/
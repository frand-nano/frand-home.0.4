use yew::*;
use frand_node::*;
use frand_web::yew::client_socket::{ClientSocket, FromServerSocket};
use super::{personal::Personal, shared::Shared};

#[node]
#[derive(Properties)]
pub struct Root {
    shared: Shared,
    personal: Personal,
}

pub enum Message {
    FromServer(Payload),
    FromNode(Payload),
}

impl From<FromServerSocket> for Message {
    fn from(value: FromServerSocket) -> Self { Self::FromServer(value.into()) }
}

impl Component for Root {
    type Message = Message;
    type Properties = Self;

    fn create(context: &Context<Self>) -> Self {
        log::debug!("Root::create");
        let socket = ClientSocket::new(context);

        let callback = context.link().callback(
            move |payload| {
                socket.send(&payload);
                Message::FromNode(payload)
            }
        );

        context.props().activate(
            move |payload| callback.emit(payload)
        ).clone()
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {    
        log::debug!("Root::view");
        html! {
            <div>
                <Shared ..self.shared.clone() />
                <Personal ..self.personal.clone() />                
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Message::FromServer(payload) => {
                log::debug!("FromServer({:?})", payload);
                self.apply_payload(&payload);
                true
            },
            Message::FromNode(payload) => {
                log::debug!("FromNode({:?})", payload);
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
    use super::{Root, RootMessage, RootMod};

    pub fn handle(
        id: &Uuid,
        send_tx: &UnboundedSender<(Uuid, Payload)>,
        broadcast_tx: &UnboundedSender<Payload>,
        node: &Root, 
        message: RootMod::Message, 
        payload: Payload,
    ) {
        use RootMessage::*;
        match message {
            shared(message) => {
                use SharedMessage::*;
                broadcast_tx.send(payload).unwrap();
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
                send_tx.send((id.clone(), payload)).unwrap();
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
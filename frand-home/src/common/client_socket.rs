use std::ops::Deref;
use frand_node::MessageData;
use yew::{Component, Context};
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

pub enum SocketMessage {
    ToServer(MessageData),
    FromServer(MessageData),
}

impl Deref for SocketMessage {
    type Target = MessageData;
    fn deref(&self) -> &Self::Target {
        match self {
            SocketMessage::ToServer(message) => message,
            SocketMessage::FromServer(message) => message,
        }
    }
}

pub struct ClientSocket {
    outbound_tx: Option<WebSocketTask>,
}

impl ClientSocket {
    pub fn new<C: Component>(context: &Context<C>) -> Self 
    where <C as yew::Component>::Message: std::convert::From<SocketMessage> 
    {
        let callback = context.link().callback(
            |message| SocketMessage::ToServer(message)
        );

        let notification = context.link().batch_callback(
            |status| match status {
                WebSocketStatus::Opened => None,
                WebSocketStatus::Closed => None,
                WebSocketStatus::Error => None,
            }
        );

        let to_server = WebSocketService::connect(
            "/ws", 
            callback,
            notification,
        );

        let outbound_tx = match to_server {
            Ok(to_server) => Some(to_server),
            Err(err) => {
                log::error!(" ClientSocket::new() -> Err({err})");
                None
            },
        };

        Self { 
            outbound_tx,
        }
    }

    pub fn send(&mut self, message: MessageData) {
        if let Some(outbound_tx) = &mut self.outbound_tx {
            outbound_tx.send_binary(message.try_into().unwrap())
        }              
    }
}
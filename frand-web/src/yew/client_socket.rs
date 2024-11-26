use frand_node::Payload;
use yew::{Component, Context};
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

pub struct FromServerSocket(Payload);

impl Into<Payload> for FromServerSocket {
    fn into(self) -> Payload { self.0 }
}

pub struct ClientSocket {
    outbound_tx: Option<WebSocketTask>,
}

impl ClientSocket {
    pub fn new<C: Component>(context: &Context<C>) -> Self 
    where <C as Component>::Message: From<FromServerSocket> 
    {
        let callback = context.link().callback(
            |payload| FromServerSocket(payload)
        );

        let notification = context.link().batch_callback(
            |status| match status {
                WebSocketStatus::Opened => { log::info!("ClientSocket Opened"); None },
                WebSocketStatus::Closed => { log::info!("ClientSocket Closed"); None },
                WebSocketStatus::Error => { log::info!("ClientSocket Error"); None },
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

    pub fn send(&self, message: &Payload) {
        if let Some(outbound_tx) = &self.outbound_tx {
            outbound_tx.send_binary(message.try_into().unwrap())
        }              
    }
}
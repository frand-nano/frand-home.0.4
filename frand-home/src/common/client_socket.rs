use std::{marker::PhantomData, ops::Deref};

use frand_node::MessageData;
use yew::{Component, Context};
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use super::simple_component::SimpleComponent;

pub struct ClientSocket<C: Component> {
    task: Option<WebSocketTask>,
    _phantom: PhantomData<C>,
}

pub enum SocketMessage<M> {
    Send(M),
    Receive(M),
}

impl<M> Deref for SocketMessage<M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        match self {
            SocketMessage::Send(message) => message,
            SocketMessage::Receive(message) => message,
        }
    }
}

impl<C: Component> ClientSocket<C> {
    pub fn new(context: &Context<SimpleComponent>) -> Self {
        let callback = context.link().callback(
            |message| SocketMessage::Send(message)
        );

        let notification = context.link().batch_callback(
            |status| match status {
                WebSocketStatus::Opened => None,
                WebSocketStatus::Closed => None,
                WebSocketStatus::Error => None,
            }
        );

        let task = WebSocketService::connect(
            "/ws", 
            callback,
            notification,
        );

        let task = match task {
            Ok(task) => Some(task),
            Err(err) => {
                log::error!(" ClientSocket::new() -> Err({err})");
                None
            },
        };

        Self { 
            task,
            _phantom: PhantomData::default(),
        }
    }

    pub fn send(&mut self, message: MessageData) {
        if let Some(task) = &mut self.task {
            task.send_binary(message.try_into().unwrap())
        }              
    }
}
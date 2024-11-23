use std::ops::{Deref, DerefMut};
use actix::spawn;
use frand_node::*;
use anyhow::Result;
use tokio::{select, sync::mpsc::{unbounded_channel, UnboundedReceiver}, task::JoinHandle};
use crate::common::simple::{Simple, SimpleMod, SimpleMessage::*, SimpleSubMessage::*};

use super::server_socket::{ServerSocket, ServerSocketMessage};

pub struct SimpleComponent {
    performer: Performer<Simple>,
    server_socket: ServerSocket,
    broadcast_rx: UnboundedReceiver<MessageData>,
}

impl Deref for SimpleComponent {
    type Target = Performer<Simple>;
    fn deref(&self) -> &Self::Target { &self.performer }
}

impl DerefMut for SimpleComponent {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.performer }
}

impl SimpleComponent {
    pub fn new(
        server_socket: ServerSocket,
    ) -> Self {
        let (broadcast_tx, broadcast_rx) = unbounded_channel::<MessageData>();

        let update = move |node: &SimpleMod::Node, message, data| {
            broadcast_tx.send(data).unwrap();

            match message {
                sub1(number1(n)) => {
                    node.sub1.number2.emit(&(n + 1));
                },
                sub1(number2(n)) => {
                    node.sub1.number3.emit(&(n + 1));
                },
                sub1(number3(n)) => {
                    node.sub1.number1.emit(&(n + 1));
                },

                sub2(number1(n)) => {
                    node.sub2.number2.emit(&(n * 2));
                    node.sub2.number3.emit(&(n / 2));
                },
                sub2(number2(n)) => {
                    node.sub2.number3.emit(&(n * 2));
                    node.sub2.number1.emit(&(n / 2));
                },
                sub2(number3(n)) => {
                    node.sub2.number1.emit(&(n * 2));
                    node.sub2.number2.emit(&(n / 2));
                },

                _ => {},
            }
        };

        Self {
            performer: Performer::<Simple>::new(update),
            server_socket,
            broadcast_rx,
        }        
    }

    pub fn run(mut self) -> JoinHandle<Result<()>> {
        spawn(async move {
            loop { select! {
                Some(message) = self.broadcast_rx.recv() => {
                    self.server_socket.broadcast(message)?;
                },
                Some(message) = self.server_socket.recv() => {
                    match message {
                        ServerSocketMessage::Open(id) => {
                            log::info!("{id} 🔗 Open");
                        },
                        ServerSocketMessage::Close((id, reason)) => {
                            log::info!("{id} 🔗 Close({:#?})", reason);                        
                        },
                        ServerSocketMessage::Message((id, message)) => {
                            log::info!("{id} 🔗 Message({:?})", message);
                            self.performer.inbound_tx().send(message)?;
                            self.performer.message_count.emit(self.performer.message_count.value());
                        },
                    }
                },     
                else => { break; }               
            }}        
            Ok(())
        })
    }
}


use frand_node::*;
use frand_web::actix::server_socket::{ServerSocket, ServerSocketMessage};
use tokio::{select, sync::mpsc::{unbounded_channel, UnboundedReceiver}, task::spawn_local};
use crate::common::simple::{Simple, SimpleMod, SimpleMessage::*, SimpleSubMessage::*};

pub struct SimpleComponent {
    simple: Performer<Simple>,
    server_socket: ServerSocket,
    broadcast_rx: UnboundedReceiver<MessageData>,
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
                    node.sub1.number2.emit(n + 1);
                },
                sub1(number2(n)) => {
                    node.sub1.number3.emit(n + 1);
                },
                sub1(number3(n)) => {
                    node.sub1.number1.emit(n + 1);
                },

                sub2(number1(n)) => {
                    node.sub2.number2.emit(n * 2);
                    node.sub2.number3.emit(n / 2);
                },
                sub2(number2(n)) => {
                    node.sub2.number3.emit(n * 2);
                    node.sub2.number1.emit(n / 2);
                },
                sub2(number3(n)) => {
                    node.sub2.number1.emit(n * 2);
                    node.sub2.number2.emit(n / 2);
                },

                _ => {},
            }
        };

        Self {
            simple: Performer::<Simple>::new(update),
            server_socket,
            broadcast_rx,
        }        
    }

    pub fn run(mut self) {
        spawn_local(async move {
            loop { select! {
                Some(message) = self.broadcast_rx.recv() => {
                    self.simple.apply(&message);
                    self.server_socket.broadcast(message);
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
                            self.simple.send(message);
                            self.simple.message_count.emit(self.simple.message_count.value() + 1);
                        },
                    }
                },     
                else => { break; }               
            }}        
        });
    }
}


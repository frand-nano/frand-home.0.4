use std::collections::HashMap;
use actix_web::web::Bytes;
use anyhow::Error;
use frand_node::MessageData;
use futures_util::StreamExt;
use actix_ws::{CloseReason, Message, MessageStream, Session};
use tokio::{select, sync::mpsc::{error::SendError, unbounded_channel, UnboundedReceiver, UnboundedSender}, task::spawn_local};
use uuid::Uuid;

pub struct ServerSocket {
    new_socket_rx: UnboundedReceiver<ServerSocketConnection>,
    socket_rx: UnboundedReceiver<ServerSocketMessage>,     
    connections: HashMap<Uuid, ServerSocketConnection>,
}

impl ServerSocket {
    pub fn new_start(
        new_socket_rx: UnboundedReceiver<ServerSocketConnection>,
        socket_rx: UnboundedReceiver<ServerSocketMessage>,     
    ) -> Self {
        Self { 
            new_socket_rx,
            socket_rx, 
            connections: HashMap::new(),
        }
    }

    pub fn send(&self, id: &Uuid, message: MessageData) -> Result<(), SendError<MessageData>> {
        self.connections[id].send(message)
    }

    pub async fn recv(&mut self) -> ServerSocketMessage {
        let result;

        loop { select! {
            Some(new_socket) = self.new_socket_rx.recv() => {
                let id = new_socket.id();
                log::info!("{id} 🔗 Open");
                self.connections.insert(id.clone(), new_socket);
            },
            Some(socket_message) = self.socket_rx.recv() => {
                match &socket_message {
                    ServerSocketMessage::Close((id, reason)) => {
                        log::info!("{id} 🔗 Close({:#?})", reason);
                        self.connections.remove(id);
                    },
                    ServerSocketMessage::Message((id, message)) => {
                        log::info!("{id} 🔗 Message({:#?})", message);
                        result = socket_message;
                        break;
                    },
                }
            }
        }}

        result
    }
}

pub struct ServerSocketConnection {
    id: Uuid,
    outbound_tx: UnboundedSender<MessageData>,      
}

pub enum ServerSocketMessage {
    Close((Uuid, Option<CloseReason>)),
    Message((Uuid, MessageData)),
}

impl ServerSocketConnection {
    pub fn id(&self) -> &Uuid { &self.id }

    pub fn new_start(
        mut stream: MessageStream,
        inbound_tx: UnboundedSender<ServerSocketMessage>,      
        mut session: Session, 
    ) -> Self {
        let id = Uuid::new_v4();
        let (outbound_tx, mut outbound_rx) = unbounded_channel();
        
        spawn_local(async move { 
            loop { select! {
                Some(message) = stream.next() => {
                    match message? {
                        Message::Binary(bytes) => {
                            inbound_tx.send(
                                ServerSocketMessage::Message(
                                    (id, MessageData::try_from(bytes.to_vec())?)
                                )
                            )?;
                        },
                        Message::Close(reason) => {
                            inbound_tx.send(
                                ServerSocketMessage::Close(
                                    (id, reason)
                                )
                            )?;
                            break;
                        },
                        _ => {},
                    }
                },
                Some(message) = outbound_rx.recv() => {
                    let message: MessageData = message;
                    let data: Vec<u8> = message.try_into()?;
                    session.binary(Bytes::copy_from_slice(data.as_slice())).await?;
                },
            }}
            Ok::<_, Error>(())
        });

        Self { 
            id, 
            outbound_tx,
        }
    }

    pub fn send(&self, message: MessageData) -> Result<(), SendError<MessageData>> {
        self.outbound_tx.send(message)
    }
}
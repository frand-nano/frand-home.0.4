use std::collections::HashMap;
use actix_web::web::Bytes;
use anyhow::Error;
use frand_node::Packet;
use futures_util::StreamExt;
use actix_ws::{CloseReason, Message, MessageStream, Session};
use tokio::{select, sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender}, task::spawn_local};
use uuid::Uuid;

pub struct ServerSocket {
    new_socket_rx: UnboundedReceiver<ServerSocketConnection>,
    socket_rx: UnboundedReceiver<ServerSocketMessage>,      
    connections: HashMap<Uuid, ServerSocketConnection>,
}

impl ServerSocket {
    pub fn new(
        new_socket_rx: UnboundedReceiver<ServerSocketConnection>,
        socket_rx: UnboundedReceiver<ServerSocketMessage>,     
    ) -> Self {
        Self { 
            new_socket_rx,
            socket_rx, 
            connections: HashMap::new(),
        }
    }

    pub fn send(&self, id: &Uuid, packet: Packet) {
        self.connections[id].send(packet);
    }

    pub fn broadcast(&self, packet: &Packet) {
        for connection in self.connections.values() {
            connection.send(packet.clone());
        }
    }

    pub async fn recv(&mut self) -> Option<ServerSocketMessage> {
        select! {
            Some(new_socket) = self.new_socket_rx.recv() => {
                let id = new_socket.id().clone();
                self.connections.insert(id.clone(), new_socket);
                Some(ServerSocketMessage::Open(id.clone()))
            },
            Some(socket_message) = self.socket_rx.recv() => { 
                match &socket_message {
                    ServerSocketMessage::Open(_) => Some(socket_message),
                    ServerSocketMessage::Message(_) => Some(socket_message),
                    ServerSocketMessage::Close((id, _)) => { 
                        self.connections.remove(id);
                        Some(socket_message) 
                    },
                }                
            },
            else => { None },
        }
    }
}

pub struct ServerSocketConnection {
    id: Uuid,
    outbound_tx: UnboundedSender<Packet>,      
}

pub enum ServerSocketMessage {
    Open(Uuid),
    Close((Uuid, Option<CloseReason>)),
    Message((Uuid, Packet)),
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
                                    (id, Packet::try_from(bytes.to_vec())?)
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
                    let message: Packet = message;
                    let data: Vec<u8> = (&message).try_into()?;
                    session.binary(Bytes::copy_from_slice(data.as_slice())).await?;
                },
                else => { break; },
            }}
            Ok::<_, Error>(())
        });

        Self { 
            id, 
            outbound_tx,
        }
    }

    pub fn send(&self, message: Packet) {
        if let Err(err) = self.outbound_tx.send(message) {
            log::error!("A closed ServerSocketConnection might not have been removed from the list. -> Err({err})")
        }
    }
}
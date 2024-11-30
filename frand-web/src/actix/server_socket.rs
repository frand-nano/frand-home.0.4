use std::collections::HashMap;
use actix_web::web::Bytes;
use anyhow::Error;
use frand_node::Packet;
use futures_util::StreamExt;
use actix_ws::{CloseReason, Message, MessageStream, Session};
use tokio::{select, sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender}, task::spawn_local};
use uuid::Uuid;

pub struct ServerSocket {
    conn_rx: UnboundedReceiver<ServerSocketConnection>,
    conn_outbound_txs: HashMap<Uuid, UnboundedSender<Packet>>,
    inbound_tx: UnboundedSender<ServerSocketMessage>,
    inbound_rx: UnboundedReceiver<ServerSocketMessage>,
}

impl ServerSocket {
    pub fn new(
        new_socket_rx: UnboundedReceiver<ServerSocketConnection>,
    ) -> Self {
        let (inbound_tx, inbound_rx) = unbounded_channel();
        Self { 
            conn_rx: new_socket_rx,
            conn_outbound_txs: HashMap::new(),
            inbound_tx,
            inbound_rx,
        }
    }

    pub fn send(&self, id: &Uuid, packet: Packet) {
        if let Err(err) = self.conn_outbound_txs[id].send(packet) {
            log::error!("A closed connection might not have been removed from the list. -> Err({err})")
        }
    }

    pub fn broadcast(&self, packet: Packet) {
        for id in self.conn_outbound_txs.keys() {
            self.send(&id, packet.clone());
        }
    }

    pub async fn recv(&mut self) -> Option<ServerSocketMessage> {
        select! {
            Some(mut conn) = self.conn_rx.recv() => {
                let id = conn.id().clone();
                let inbound_tx = self.inbound_tx.clone();
                let (conn_outbound_tx, mut conn_outbound_rx) = unbounded_channel();      

                self.conn_outbound_txs.insert(id, conn_outbound_tx);

                spawn_local(async move { 
                    loop { select! {
                        Some(packet) = conn_outbound_rx.recv() => {
                            conn.outbound_tx().send(packet).unwrap();
                        }
                        Some(message) = conn.inbound_rx().recv() => {
                            inbound_tx.send(message).unwrap();
                        }
                    }}
                });

                Some(ServerSocketMessage::Open(id))
            },
            Some(socket_message) = self.inbound_rx.recv() => { 
                match &socket_message {
                    ServerSocketMessage::Open(_) => Some(socket_message),
                    ServerSocketMessage::Message(_) => Some(socket_message),
                    ServerSocketMessage::Close((id, _)) => { 
                        self.conn_outbound_txs.remove(id);
                        Some(socket_message) 
                    },
                }                
            },
            else => { None }
        }
    }
}

#[derive(Debug)]
pub struct ServerSocketConnection {
    id: Uuid,
    inbound_rx: UnboundedReceiver<ServerSocketMessage>,    
    outbound_tx: UnboundedSender<Packet>,      
}

pub enum ServerSocketMessage {
    Open(Uuid),
    Close((Uuid, Option<CloseReason>)),
    Message((Option<Uuid>, Packet)),
}

impl ServerSocketConnection {
    pub fn id(&self) -> &Uuid { &self.id }
    pub fn inbound_rx(&mut self) -> &mut UnboundedReceiver<ServerSocketMessage> { &mut self.inbound_rx }
    pub fn outbound_tx(&self) -> &UnboundedSender<Packet> { &self.outbound_tx }

    pub fn new_start(
        mut stream: MessageStream,  
        mut session: Session, 
    ) -> Self {
        let id = Uuid::new_v4();
        let (inbound_tx, inbound_rx) = unbounded_channel();
        let (outbound_tx, mut outbound_rx) = unbounded_channel();
        
        spawn_local(async move { 
            loop { select! {
                Some(message) = stream.next() => {
                    match message? {
                        Message::Binary(bytes) => {
                            inbound_tx.send(
                                ServerSocketMessage::Message(
                                    (Some(id), Packet::try_from(bytes.to_vec())?)
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
                }
                Some(packet) = outbound_rx.recv() => {
                    let packet: Packet = packet;
                    let data: Vec<u8> = packet.try_into()?;
                    session.binary(Bytes::copy_from_slice(data.as_slice())).await?;
                }
                else => { break; }
            }}
            Ok::<_, Error>(())
        });

        Self { 
            id, 
            inbound_rx,
            outbound_tx,
        }
    }
}
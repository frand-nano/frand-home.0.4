use std::collections::HashMap;
use bases::AsyncProcessor;
use frand_node::*;
use frand_web::actix::server_socket::{ServerSocketConnection, ServerSocketMessage};
use tokio::{select, sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender}, task::spawn_local};
use uuid::Uuid;
use crate::app::node::root::Root;

use super::{client::Client, server::Server};

pub struct ActixApp {
    new_conn_rx: UnboundedReceiver<ServerSocketConnection>,
}

impl ActixApp {
    pub fn new(
        new_conn_rx: UnboundedReceiver<ServerSocketConnection>,
    ) -> Self {
        Self {
            new_conn_rx,
        }        
    }

    pub fn start(self) {        
        let (send_tx, mut send_rx) = unbounded_channel::<(Option<Uuid>, Packet)>();
        let (server_inbound_tx, mut server_inbound_rx) = unbounded_channel::<(Option<Uuid>, Packet)>();

        spawn_local(async move {
            let mut server = Server::new(send_tx);
            while let Some((id, packet)) = server_inbound_rx.recv().await {
                server.process(id, packet).await;
            }
        });

        spawn_local(async move {
            let mut new_conn_rx = self.new_conn_rx;
            let mut conn_outbound_txs = HashMap::new();

            let (client_inbound_tx, mut client_inbound_rx) = unbounded_channel::<ServerSocketMessage>();

            loop { select! {
                Some(new_conn) = new_conn_rx.recv() => {
                    conn_outbound_txs.insert(new_conn.id().clone(), new_conn.outbound_tx().clone());
                    Self::start_client_process(new_conn, client_inbound_tx.clone());
                }
                Some((id, packet)) = send_rx.recv() => {
                    match &id {
                        Some(id) => conn_outbound_txs[&id].send(packet).unwrap(),
                        None => {
                            for conn_outbound_tx in conn_outbound_txs.values() {
                                conn_outbound_tx.send(packet.clone()).unwrap();
                            }
                        },
                    }
                }
                Some(message) = client_inbound_rx.recv() => {
                    match message {
                        ServerSocketMessage::Open(_) => {},
                        ServerSocketMessage::Close((id, _)) => {
                            conn_outbound_txs.remove(&id);
                        },
                        ServerSocketMessage::Message((id, packet)) => {
                            server_inbound_tx.send((id, packet)).unwrap();
                        },
                    }
                }
                else => { break; }        
            }}        
        });
    }

    fn start_client_process(
        mut conn: ServerSocketConnection,
        output_tx: UnboundedSender<ServerSocketMessage>,
    ) {
        let id = *conn.id();
        let (processor, mut processed_rx) = AsyncProcessor::<Root>::new();
        let mut client = Client::new(processor);

        let output_tx_clone = output_tx.clone();
        spawn_local(async move {
            while let Some(message) = conn.inbound_rx().recv().await {
                match message {
                    ServerSocketMessage::Open(id) => {
                        log::info!("{id} 🔗 Open");
                    },
                    ServerSocketMessage::Close((id, reason)) => {
                        log::info!("{id} 🔗 Close({:#?})", reason);      
                        output_tx_clone.send(ServerSocketMessage::Close((id, reason))).unwrap();         
                        break;
                    },
                    ServerSocketMessage::Message((id, packet)) => {
                        log::info!("{:?} 🔗 Message({:?})", id, packet);
                        client.process(packet).await;
                    },
                }
            }
        });

        spawn_local(async move {
            while let Some(packet) = processed_rx.recv().await {
                output_tx.send(ServerSocketMessage::Message((Some(id), packet))).unwrap();
            }
        });
    }
}


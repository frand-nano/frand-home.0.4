use std::collections::HashMap;
use frand_node::*;
use frand_web::actix::server_socket::{ServerSocket, ServerSocketMessage};
use tokio::{select, sync::mpsc::{unbounded_channel, UnboundedSender}, task::spawn_local};
use uuid::Uuid;
use crate::app::root::{backend::handle, Root};

pub struct ActixApp {
    client_nodes: HashMap<Uuid, Root>,
    server_socket: ServerSocket,
}

impl ActixApp {
    pub fn new(
        server_socket: ServerSocket,
    ) -> Self {
        Self {
            client_nodes: HashMap::new(),
            server_socket,
        }        
    }

    pub fn run(mut self) {
        let (send_tx, mut send_rx) = unbounded_channel::<(Uuid, Payload)>();
        let (broadcast_tx, mut broadcast_rx) = unbounded_channel::<Payload>();
        
        spawn_local(async move {
            loop { select! {
                Some((id, payload)) = send_rx.recv() => {
                    self.client_nodes.get_mut(&id).unwrap().apply(&payload);
                    self.server_socket.send(&id, payload);
                },
                Some(payload) = broadcast_rx.recv() => {
                    for node in self.client_nodes.values_mut() {
                        node.apply(&payload);
                    }
                    self.server_socket.broadcast(payload);
                },
                Some(message) = self.server_socket.recv() => {
                    match message {
                        ServerSocketMessage::Open(id) => {
                            log::info!("{id} 🔗 Open");
                            
                            self.client_nodes.insert(id, 
                                Self::new_yew_node(id, send_tx.clone(), broadcast_tx.clone()),
                            );
                        },
                        ServerSocketMessage::Close((id, reason)) => {
                            log::info!("{id} 🔗 Close({:#?})", reason);          
                            self.client_nodes.remove(&id);              
                        },
                        ServerSocketMessage::Message((id, payload)) => {
                            log::info!("{id} 🔗 Message({:?})", payload);
                            self.client_nodes[&id].callback().send(payload).unwrap();
                        },
                    }
                },     
                else => { break; }               
            }}        
        });
    }

    fn new_yew_node(
        id: Uuid,
        send_tx: UnboundedSender<(Uuid, Payload)>,
        broadcast_tx: UnboundedSender<Payload>,
    ) -> Root {
        Processor::<Root>::new_node(
            move |node: &Root, message, payload| {
                handle(&id, &send_tx, &broadcast_tx, node, message, payload)
            }
        )
    }
}

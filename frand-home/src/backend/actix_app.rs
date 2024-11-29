use std::collections::HashMap;
use frand_node::*;
use frand_web::actix::server_socket::{ServerSocket, ServerSocketMessage};
use tokio::{select, sync::mpsc::{unbounded_channel, UnboundedSender}, task::spawn_local};
use uuid::Uuid;
use crate::app::{app::{Root, RootMessage}, personal::PersonalMessage, shared::SharedMessage};

pub struct ActixApp {
    clients: HashMap<Uuid, Container<Root>>,
    server_socket: ServerSocket,
}

impl ActixApp {
    pub fn new(
        server_socket: ServerSocket,
    ) -> Self {
        Self {
            clients: HashMap::new(),
            server_socket,
        }        
    }

    pub fn run(mut self) {
        let (client_tx, mut client_rx) = unbounded_channel::<(Uuid, Packet)>();
        
        spawn_local(async move {
            loop { select! {
                Some((id, packet)) = client_rx.recv() => {
                    Self::handle_message(
                        self.clients.get_mut(&id).unwrap(), 
                        &id, 
                        packet,
                        &mut self.server_socket,
                    );                    
                },
                Some(message) = self.server_socket.recv() => {
                    match message {
                        ServerSocketMessage::Open(id) => {
                            log::info!("{id} 🔗 Open");
                            
                            self.clients.insert(id, 
                                Self::new_client(id, client_tx.clone()),
                            );
                        },
                        ServerSocketMessage::Close((id, reason)) => {
                            log::info!("{id} 🔗 Close({:#?})", reason);          
                            self.clients.remove(&id);              
                        },
                        ServerSocketMessage::Message((id, packet)) => {
                            log::info!("{id} 🔗 Message({:?})", packet);
                            self.clients[&id].emit_packet(packet);
                        },
                    }
                },     
                else => { break; }               
            }}        
        });
    }

    fn new_client(
        id: Uuid,
        client_tx: UnboundedSender<(Uuid, Packet)>,
    ) -> Container<Root> {
        Container::new(move |packet| client_tx.send((id, packet)).unwrap())      
    }

    fn handle_message(
        client: &mut Container<Root>, 
        id: &Uuid, 
        packet: Packet,
        socket: &mut ServerSocket,
    ) {
        client.process(packet, |node, packet, message| {
            use RootMessage::*;
            match message {
                shared(message) => {
                    use SharedMessage::*;
                    socket.broadcast(packet);
                    match message {
                        number1(n) => node.shared.number2.emit(n+1),
                        number2(n) => node.shared.number3.emit(n+1),
                        number3(n) => node.shared.number4.emit(n+1),
                        number4(n) => node.shared.number1.emit(n+1),
                        _ => {},
                    }
                },
                personal(message) => {
                    use PersonalMessage::*;
                    socket.send(&id, packet.clone());
                    match message {
                        number1(n) => node.personal.number2.emit(n+1),
                        number2(n) => node.personal.number3.emit(n+1),
                        number3(n) => node.personal.number4.emit(n+1),
                        number4(n) => node.personal.number1.emit(n+1),
                        _ => {},
                    }
                },
                _ => {},
            }
        });
    }
}


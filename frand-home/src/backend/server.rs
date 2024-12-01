use frand_node::*;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;
use crate::app::node::root::RootMessage;

pub struct Server {
    send_tx: UnboundedSender<(Option<Uuid>, Packet)>,
}

impl Server {
    pub fn new(send_tx: UnboundedSender<(Option<Uuid>, Packet)>) -> Self {
        Self { 
            send_tx,
        }
    }

    pub async fn process(&mut self, id: Option<Uuid>, packet: Packet) {     
        use RootMessage::*;   

        let message = Message::from_packet(0, &packet);

        match message {
            shared(_) => self.send_tx.send((None, packet)).unwrap(),
            personal(_) => self.send_tx.send((id, packet)).unwrap(),
            _ => {},
        }
    }
}
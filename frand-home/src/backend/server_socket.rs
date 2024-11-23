use actix_web::web::Bytes;
use anyhow::Error;
use frand_node::MessageData;
use futures_util::StreamExt;
use actix_ws::{Message, MessageStream, Session};
use tokio::{select, sync::mpsc::{unbounded_channel, UnboundedSender}, task::spawn_local};
use uuid::Uuid;

pub struct ServerSocketConnection {
    id: Uuid,
    outbound_tx: UnboundedSender<MessageData>,      
}

pub struct ServerSocketMessage {
    pub id: Uuid,
    pub message: MessageData,
}

impl ServerSocketConnection {
    pub fn id(&self) -> &Uuid { &self.id }

    pub fn new_run(
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
                        Message::Binary(bytes) => inbound_tx.send(ServerSocketMessage {
                            id,
                            message: MessageData::try_from(bytes.to_vec())?,
                        })?,
                        Message::Close(reason) => {
                            log::info!("Socket {id} Closed. reason: {:?}", reason);
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
}
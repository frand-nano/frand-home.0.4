use std::time::Duration;

use frand_node::*;
use tokio::time::sleep;

use crate::app::node::{personal::PersonalMessage, root::{Root, RootMessage}, shared::SharedMessage};

pub struct Client {
    processor: AsyncProcessor<Root>,
}

impl Client {
    pub fn new(processor: AsyncProcessor<Root>) -> Self {
        Self { 
            processor,
        }
    }

    pub async fn process(&mut self, packet: Packet) {
        self.processor.process(packet, |node, _packet, message| {
            use RootMessage::*;
            match message {
                shared(message) => {
                    use SharedMessage::*;
                    match message {
                        number1(n) => node.shared.number2.emit_future(async move {
                            sleep(Duration::from_millis(200)).await;
                            n+1
                        }),
                        number2(n) => node.shared.number3.emit_future(async move {
                            sleep(Duration::from_millis(200)).await;
                            n+1
                        }),
                        number3(n) => node.shared.number4.emit_future(async move {
                            sleep(Duration::from_millis(200)).await;
                            n+1
                        }),
                        number4(n) => node.shared.number1.emit_future(async move {
                            sleep(Duration::from_millis(200)).await;
                            n+1
                        }),
                        _ => {},
                    }
                },
                personal(message) => {
                    use PersonalMessage::*;
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
        }).await;
    }
}
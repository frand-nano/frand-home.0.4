use frand_node::*;
use crate::app::node::{number_sum::NumberSumMessage, personal::PersonalMessage, root::{Root, RootMessage}, shared::SharedMessage};

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
            use NumberSumMessage::*;
            match message {
                shared(message) => {
                    use SharedMessage::*;
                    match message {
                        sum1(a(_) | b(_)) => node.shared.sum1.emit_expensive_sum(),
                        sum1(sum(s)) => node.shared.sum3.a.emit(s),

                        sum2(a(_) | b(_)) => node.shared.sum2.emit_expensive_sum(),
                        sum2(sum(s)) => node.shared.sum3.b.emit(s),

                        sum3(a(_) | b(_)) => node.shared.sum3.emit_expensive_sum(),
                        _ => {},
                    }
                },
                personal(message) => {
                    use PersonalMessage::*;
                    match message {
                        sum1(a(_) | b(_)) => node.personal.sum1.emit_expensive_sum(),
                        sum1(sum(s)) => node.personal.sum3.a.emit(s),

                        sum2(a(_) | b(_)) => node.personal.sum2.emit_expensive_sum(),
                        sum2(sum(s)) => node.personal.sum3.b.emit(s),

                        sum3(a(_) | b(_)) => node.personal.sum3.emit_expensive_sum(),
                        _ => {},
                    }
                },
                _ => {},
            }
        }).await;
    }
}
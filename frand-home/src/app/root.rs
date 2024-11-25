use frand_node::*;
use yew::{html, Html};
use super::{personal::Personal, shared::Shared};

#[node]
#[derive(yew::Properties)]
pub struct Root {
    pub shared: Shared,
    pub personal: Personal,
}

#[cfg(not(target_arch = "wasm32"))]
pub mod backend {
    use frand_node::*;
    use uuid::Uuid;
    use tokio::sync::mpsc::UnboundedSender;
    use crate::app::{personal::PersonalMessage, shared::SharedMessage};
    use super::{Root, RootMessage, RootMod};

    pub fn handle(
        id: &Uuid,
        send_tx: &UnboundedSender<(Uuid, Payload)>,
        broadcast_tx: &UnboundedSender<Payload>,
        node: &Root, 
        message: RootMod::Message, 
        payload: Payload,
    ) {
        use RootMessage::*;
        match message {
            shared(message) => {
                use SharedMessage::*;
                broadcast_tx.send(payload).unwrap();
                match message {
                    number1(n) => node.shared.number2.emit(n + 1),
                    number2(n) => node.shared.number3.emit(n + 1),
                    number3(n) => node.shared.number4.emit(n + 1),
                    number4(n) => node.shared.number1.emit(n + 1),
                    _ => {},
                }
            }
            personal(message) => {
                use PersonalMessage::*;
                send_tx.send((id.clone(), payload)).unwrap();
                match message {
                    number1(n) => {
                        node.personal.number4.emit(n - 1);
                        node.personal.number2.emit(n + 1);
                    },
                    number2(n) => {
                        node.personal.number1.emit(n - 1);
                        node.personal.number3.emit(n + 1);
                    },
                    number3(n) => {
                        node.personal.number2.emit(n - 1);
                        node.personal.number4.emit(n + 1);
                    },
                    number4(n) => {
                        node.personal.number3.emit(n - 1);
                        node.personal.number1.emit(n + 1);
                    },
                    _ => {},
                }
            }
            _ => {},
        }
    }
}

#[yew::function_component]
pub fn RootView(root: &Root) -> Html {
    let add1 = |node: Node<i32>| {
        (
            *node.value(),
            move |_| node.emit(node.value() + 1), 
        )
    };

    let sn1 = (add1)(root.shared.number1.clone());
    let sn2 = (add1)(root.shared.number2.clone());
    let sn3 = (add1)(root.shared.number3.clone());
    let sn4 = (add1)(root.shared.number4.clone());

    let cn1 = (add1)(root.personal.number1.clone());
    let cn2 = (add1)(root.personal.number2.clone());
    let cn3 = (add1)(root.personal.number3.clone());
    let cn4 = (add1)(root.personal.number4.clone());

    html! {
        <div>
            <div>
                <button onclick = {sn1.1}>
                    { format!("sn1 : {} + 1", sn1.0) }
                </button>
                <button onclick = {sn2.1}>
                    { format!("sn2 : {} + 1", sn2.0) }
                </button>
                <button onclick = {sn3.1}>
                    { format!("sn3 : {} + 1", sn3.0) }
                </button>
                <button onclick = {sn4.1}>
                    { format!("sn4 : {} + 1", sn4.0) }
                </button>
            </div>
            <div>
                <button onclick = {cn1.1}>
                    { format!("cn1 : {} + 1", cn1.0) }
                </button>
                <button onclick = {cn2.1}>
                    { format!("cn2 : {} + 1", cn2.0) }
                </button>
                <button onclick = {cn3.1}>
                    { format!("cn3 : {} + 1", cn3.0) }
                </button>
                <button onclick = {cn4.1}>
                    { format!("cn4 : {} + 1", cn4.0) }
                </button>
            </div>
        </div>
    }
}
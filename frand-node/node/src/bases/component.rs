use super::{state::StateBase, MessageBase, NodeBase};

pub trait ComponentBase: 'static {
    type State: StateBase;
    type Node: NodeBase;
    type Message: MessageBase;

    fn node(&self) -> &Self::Node;
        
    fn new() -> Self;
}

pub trait Component: ComponentBase {
    fn update(node: &Self::Node, message: Self::Message) -> anyhow::Result<()>;
}
use crossbeam::channel::Sender;

use crate::{
    bases::{
        message::MessageData, 
        node::NodeBase, CallbackSender, Processor, 
    },
    result::Result,
};

use super::{MessageBase, StateBase};

pub trait Update: 'static {
    type Node: NodeBase;
    type Message: MessageBase;

    fn update(node: &Self::Node, message: Self::Message) -> anyhow::Result<()>;
}

pub struct Performer<S: StateBase> {     
    node: S::Node,     
    callback: CallbackSender,
    input: Sender<MessageData>, 
}

impl<S: 'static + StateBase> Performer<S> {
    pub fn node(&self) -> &S::Node { &self.node }
    pub fn input(&self) -> &Sender<MessageData> { &self.input }

    pub fn new<U>(update: U) -> Self 
    where U: 'static + Fn(&S::Node, S::Message, MessageData) -> anyhow::Result<()>
    {
        log::info!("Performer new");
        
        let (callback, input) = Processor::<S, U>::new_callback(update);
        let callback = CallbackSender::Callback(callback);

        Self { 
            node: S::Node::new(&callback, vec![], None), 
            callback,
            input,
        }      
    }

    pub fn replace_node(&mut self, node: &S::Node) {
        log::info!("Performer replace_node");

        node.reset_sender(&self.callback);
        self.node = node.clone();
    }

    pub fn apply(&mut self, message: MessageData) -> Result<()> {
        Ok(self.node.apply(message)?)
    }

    pub fn apply_messages<I>(&mut self, messages: I) -> Result<()> 
    where I: Iterator<Item = MessageData>
    {
        Ok(for message in messages {
            self.node.apply(message)?;
        })
    }
}

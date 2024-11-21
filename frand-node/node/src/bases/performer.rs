use crossbeam::channel::{Receiver, Sender};

use crate::{
    bases::{
        message::MessageData, 
        node::NodeBase, CallbackSender, Processor, 
    },
    result::Result,
};

use super::{Component, ComponentBase};

pub struct Performer<C: ComponentBase> {     
    node: C::Node,     
    callback: CallbackSender,
    input: Sender<MessageData>, 
    output: Receiver<MessageData>,
}

impl<C: ComponentBase> Performer<C> {
    pub fn node(&self) -> &C::Node { &self.node }
    pub fn input(&self) -> &Sender<MessageData> { &self.input }
    pub fn output(&self) -> &Receiver<MessageData> { &self.output }

    pub fn new() -> Self
    where C: Component
    {
        log::info!("Performer new");
        
        let (callback, input, output) = Processor::<C>::new_callback();
        let callback = CallbackSender::Callback(callback);

        Self { 
            node: C::Node::new(&callback, vec![], None), 
            callback,
            input,
            output,
        }      
    }

    pub fn replace_node(&mut self, node: &C::Node) {
        log::info!("Performer replace_node");

        node.reset_sender(&self.callback);
        self.node = node.clone();
    }

    pub fn apply(&mut self, messages : Receiver<MessageData>) -> Result<()> {
        Ok(for message in messages.try_iter() {
            self.node.apply(message)?;
        })
    }

    pub fn apply_output(&mut self) -> Result<()> {
        Ok(for message in self.output.try_iter() {
            self.node.apply(message)?;
        })
    }
}

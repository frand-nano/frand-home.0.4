use crossbeam::channel::Sender;
use crate::{*,
    bases::{message::MessageData, CallbackSender, Processor},
    result::Result,
};

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
        let (callback, input) = Processor::<S, U>::new_callback(update);
        let callback = CallbackSender::Callback(callback);

        Self { 
            node: S::Node::new(&callback, vec![], None), 
            callback,
            input,
        }      
    }

    pub fn new_with<U>(node: &S::Node, update: U) -> Self 
    where U: 'static + Fn(&S::Node, S::Message, MessageData) -> anyhow::Result<()>
    {
        let mut result = Self::new(update);
        node.reset_sender(&result.callback);
        result.node = node.clone();
        result
    }

    pub fn apply(&mut self, message: &MessageData) -> Result<()> {
        self.node.apply(message)
    }

    pub fn apply_messages<I>(&mut self, messages: I) -> Result<()> 
    where 
        I: Iterator<Item = MessageData>,
        I::Item: AsRef<MessageData>,
    {
        Ok(for message in messages {
            self.node.apply(message.as_ref())?;
        })
    }
}

use std::{ops::{Deref, DerefMut}, sync::Arc, fmt::Debug};
use crate::{*,
    result::Result,
};

#[derive(Clone)]
pub struct ContainerCallback(pub Arc<dyn Fn(MessageData) -> Result<()>>);

impl Debug for ContainerCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ContainerCallback(Arc<dyn Fn(MessageData) -> Result<()>>)")
    }
}

impl Deref for ContainerCallback {
    type Target = Arc<dyn Fn(MessageData) -> Result<()>>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for ContainerCallback {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

pub trait ContainerBase<S: StateBase>: Deref<Target = S::Node> + DerefMut {
    
}
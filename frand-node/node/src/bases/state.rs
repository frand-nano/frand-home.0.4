use serde::{de::DeserializeOwned, Serialize};
use super::ElementBase;

pub trait StateBase: 'static + ElementBase + Serialize + DeserializeOwned {
    
} 
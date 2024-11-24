use serde::{de::DeserializeOwned, Serialize};
use super::ElementBase;

pub trait StateBase: ElementBase + Serialize + DeserializeOwned {
    
} 
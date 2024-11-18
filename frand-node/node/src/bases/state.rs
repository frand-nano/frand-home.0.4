use serde::{de::DeserializeOwned, Serialize};

pub trait State: Default + PartialEq + Serialize + DeserializeOwned {
    
}
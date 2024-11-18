use std::io::Cursor;
use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

pub const MESSAGE_ID_SELF: usize = usize::MAX - 1;

/// ```
/// # use frand_node::bases::{message::ComponentMessageData, state::State};
/// #
/// let mut data = ComponentMessageData::new(
///     &vec![1,2], 
///     &String::from("foo")
/// );
/// 
/// assert_eq!(data.read()?, (vec![1,2], String::from("foo")));
/// #
/// # Ok::<(), anyhow::Error>(())
/// ```
pub struct ComponentMessageData {
    ids: Vec<u8>,
    state: Vec<u8>,
}

impl ComponentMessageData {
    pub fn new<S: Serialize>(ids: &Vec<usize>, state: &S) -> Result<Self> {
        let mut result = Self { 
            ids: Vec::new(), 
            state: Vec::new(), 
        };

        ciborium::into_writer(ids, &mut result.ids)?;
        ciborium::into_writer(state, &mut result.state)?;

        Ok(result)
    }

    pub fn read<S: DeserializeOwned>(&self) -> Result<(Vec<usize>, S)> {
        let ids = ciborium::from_reader(Cursor::new(&self.ids))?;
        let state = ciborium::from_reader(Cursor::new(&self.state))?;

        Ok((ids, state))
    }
}
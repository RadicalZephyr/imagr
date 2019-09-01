use std::fmt;

use serde::de::DeserializeOwned;
use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(bound = "T: Clone + fmt::Debug + DeserializeOwned")]
pub struct Response<T>
where
    T: Clone + fmt::Debug + DeserializeOwned,
{
    pub meta: Meta,
    pub response: T,
}
#[derive(Clone, Debug, Deserialize)]
pub struct Meta {
    pub status: u16,
    pub msg: String,
}

impl Meta {
    pub fn is_success(&self) -> bool {
        self.status == 200
    }
}

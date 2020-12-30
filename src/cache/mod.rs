pub mod redis;

use std::fmt::Display;
use std::time::Duration;

use serde::{de::DeserializeOwned, ser::Serialize};

use super::errors::Result;

pub trait Provider {
    fn get<K, V, F>(&self, key: &K, fun: F, ttl: Duration) -> Result<V>
    where
        F: FnOnce() -> Result<V>,
        K: Display,
        V: DeserializeOwned + Serialize;
    fn clear(&self) -> Result<()>;
}

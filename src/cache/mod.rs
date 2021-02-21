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
    fn keys(&self) -> Result<Vec<(String, i64)>>;
    fn version(&self) -> Result<String>;
}

pub trait KV {
    fn set<K: Display, V: Serialize>(&self, key: &K, val: &V) -> Result<()>;
    fn get<K: Display, V: DeserializeOwned>(&self, key: &K) -> Result<V>;
}

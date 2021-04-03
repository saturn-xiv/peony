use std::default::Default;
use std::fmt;
use std::fmt::Display;
use std::ops::DerefMut;
use std::time::Duration;

use redis_::{cmd, Commands, RedisResult};
use serde::{de::DeserializeOwned, ser::Serialize};

use super::super::errors::Result;

pub type Connection = redis_::Client;
pub type Pool = r2d2::Pool<Connection>;
pub type PooledConnection = r2d2::PooledConnection<Connection>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub user: Option<String>,
    pub password: Option<String>,
    pub port: Option<u16>,
    pub db: Option<u8>,
    pub pool: Option<u32>,
}

impl Config {
    const PORT: u16 = 6379;
    const DB: u8 = 0;
    pub fn open(&self) -> Result<Pool> {
        let manager = Connection::open(self.to_string())?;
        let pool = r2d2::Pool::builder()
            .max_size(self.pool.unwrap_or(32))
            .build(manager)?;
        Ok(pool)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: Some(Self::PORT),
            user: None,
            password: None,
            db: Some(Self::DB),
            pool: Some(32),
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "redis://{}:{}@{}:{}/{}",
            self.user.as_deref().unwrap_or(""),
            self.password.as_deref().unwrap_or(""),
            self.host,
            self.port.unwrap_or(Self::PORT),
            self.db.unwrap_or(Self::DB)
        )
    }
}

// https://redis.io/commands
impl super::Provider for Pool {
    fn version(&self) -> Result<String> {
        let mut db = self.get()?;
        let db = db.deref_mut();
        let it = cmd("info").query::<String>(db)?;
        Ok(it)
    }
    fn keys(&self) -> Result<Vec<(String, i64)>> {
        let mut db = self.get()?;
        let db = db.deref_mut();
        let mut items = Vec::new();
        let keys: Vec<String> = db.keys("*")?;
        for it in keys {
            let ttl = db.ttl(&it)?;
            items.push((it, ttl));
        }
        Ok(items)
    }

    fn get<K, V, F>(&self, key: &K, fun: F, ttl: Duration) -> Result<V>
    where
        F: FnOnce() -> Result<V>,
        K: Display,
        V: DeserializeOwned + Serialize,
    {
        let mut db = self.get()?;
        let db = db.deref_mut();

        let key = key.to_string();
        let buf: RedisResult<Vec<u8>> = db.get(&key);
        if let Ok(buf) = buf {
            if let Ok(val) = flexbuffers::from_slice(buf.as_slice()) {
                return Ok(val);
            }
        }
        debug!("cache expire, set {:?} {:?}", key, ttl);
        let val = fun()?;
        let _: () = db.set_ex(
            &key,
            flexbuffers::to_vec(&val)?.as_slice(),
            ttl.as_secs() as usize,
        )?;
        Ok(val)
    }

    fn clear(&self) -> Result<()> {
        let mut db = self.get()?;
        let db = db.deref_mut();
        let rst = cmd("flushdb").query::<String>(db)?;
        info!("{}", rst);
        Ok(())
    }
}

impl super::Kv for Pool {
    fn set<K: Display, V: Serialize>(&self, key: &K, val: &V) -> Result<()> {
        let key = key.to_string();
        let val = flexbuffers::to_vec(val)?;
        let mut db = self.get()?;
        let db = db.deref_mut();
        let _: () = db.set(&key, val.as_slice())?;
        Ok(())
    }
    fn get<K: Display, V: DeserializeOwned>(&self, key: &K) -> Result<V> {
        let key = key.to_string();
        let mut db = self.get()?;
        let db = db.deref_mut();

        let val: Vec<u8> = db.get(&key)?;
        let val = flexbuffers::from_slice(val.as_slice())?;
        Ok(val)
    }
}

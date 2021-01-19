use std::default::Default;
use std::fmt;
use std::fmt::Display;
use std::ops::DerefMut;
use std::time::Duration;

use redis_::cmd;
use serde::{de::DeserializeOwned, ser::Serialize};

use super::super::errors::Result;

pub type Connection = redis_::Client;
pub type Pool = r2d2::Pool<Connection>;
pub type PooledConnection = r2d2::PooledConnection<Connection>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db: u8,
    pub pool: Option<u32>,
}

impl Config {
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
            port: 6379,
            db: 0,
            pool: None,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "redis://{}:{}/{}", self.host, self.port, self.db)
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
        for key in cmd("keys").arg("*").query::<Vec<String>>(db)? {
            let ttl = cmd("ttl").arg(&key).query::<i64>(db)?;
            items.push((key, ttl));
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
        if let Ok(buf) = cmd("get").arg(&key).query::<Vec<u8>>(db) {
            if let Ok(val) = serde_json::from_slice(buf.as_slice()) {
                return Ok(val);
            }
        }
        debug!("set {:?} {:?}", key, ttl);
        let val = fun()?;
        let _: String = cmd("set")
            .arg(&key)
            .arg(serde_json::to_vec(&val)?.as_slice())
            .arg("ex")
            .arg(ttl.as_secs())
            .query(db)?;
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

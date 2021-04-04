use std::default::Default;
use std::fmt;

use diesel::{prelude::*, sql_query};

use super::super::errors::Result;
use super::migration::{Dao, Version};

// https://www.postgresql.org/docs/current/runtime-config-logging.html
// /var/lib/postgres/data/postgresql.conf: log_statement = 'all'
// journalctl -f -u postgresql
pub type Connection = diesel::pg::PgConnection;
pub type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<Connection>>;
pub type PooledConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<Connection>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: Option<String>,
    pub pool: Option<u32>,
}

impl Config {
    pub fn open(&self) -> Result<Pool> {
        let manager = diesel::r2d2::ConnectionManager::<Connection>::new(&self.to_string());
        Ok(Pool::builder()
            .max_size(self.pool.unwrap_or(32))
            .build(manager)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            name: "dev".to_string(),
            password: None,
            pool: None,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password.as_deref().unwrap_or(""),
            self.host,
            self.port,
            self.name
        )
    }
}

impl Dao for Connection {
    fn version(&self) -> Result<String> {
        let it: Version = sql_query("SELECT VERSION() AS value").get_result(self)?;
        Ok(it.value)
    }
}

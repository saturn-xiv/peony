use std::default::Default;
use std::fmt;

use super::{
    cache::redis::{Config as RedisConfig, Pool as CachePool},
    crypto::Key,
    errors::Result,
    jwt::Jwt,
    orm::postgresql::{Config as PostgreSqlConfig, Pool as DbPool},
    queue::rabbit::{Config as RabbitMQConfig, RabbitMQ},
    twilio::Config as TwilioConfig,
};

include!(concat!(env!("OUT_DIR"), "/env.rs"));

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const BANNER: &str = include_str!("banner.txt");

pub const LOCALHOST: &str = "127.0.0.1";

pub struct Context {
    pub cache: CachePool,
    pub db: DbPool,
    pub jwt: Jwt,
    pub queue: RabbitMQ,
}

impl Context {
    pub fn new(cfg: &Config) -> Result<Self> {
        Ok(Self {
            cache: cfg.redis.open()?,
            db: cfg.postgresql.open()?,
            jwt: Jwt::new(cfg.secrets.0.clone()),
            queue: cfg.rabbitmq.open(),
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Environment {
    Production,
    Development,
    Test,
}

impl Default for Environment {
    fn default() -> Self {
        Self::Development
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::Production => fmt.write_str("production"),
            Environment::Development => fmt.write_str("development"),
            Environment::Test => fmt.write_str("test"),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Config {
    pub env: Environment,
    pub secrets: Key,
    pub http: Http,
    pub grpc: Grpc,
    pub postgresql: PostgreSqlConfig,
    pub redis: RedisConfig,
    pub rabbitmq: RabbitMQConfig,
    pub twilio: TwilioConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Http {
    pub origins: Vec<String>,
    pub port: u16,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            port: 8080,
            origins: vec!["https://www.change-me.com".to_string()],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grpc {
    pub port: u16,
    pub threads: usize,
    pub memory: usize,
}

impl Default for Grpc {
    fn default() -> Self {
        Self {
            port: 8080,
            threads: 8,
            memory: 10,
        }
    }
}

impl fmt::Display for Grpc {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}:{}", LOCALHOST, self.port)
    }
}

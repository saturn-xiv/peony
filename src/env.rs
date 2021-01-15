use std::collections::HashMap;
use std::default::Default;
use std::fmt;

use super::{
    cache::redis::{Config as RedisConfig, Pool as CachePool},
    crypto::Key,
    errors::Result,
    jwt::Jwt,
    mailer::{Config as Mailer, Task as MailerTask},
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
    pub administrators: HashMap<String, Administrator>,
    pub http: Http,
    pub grpc: Grpc,
    pub postgresql: PostgreSqlConfig,
    pub redis: RedisConfig,
    pub rabbitmq: RabbitMQConfig,
    pub twilio: TwilioConfig,
    pub mailer: Mailer,
}

impl Config {
    pub async fn alert(&self, subject: &str, body: &str) -> Result<()> {
        for (_, to) in self.administrators.iter() {
            if let Some(ref to) = to.phone {
                self.twilio.sms(to, body, None).await?;
            }
            if let Some(ref to) = to.email {
                let qu = self.rabbitmq.open();

                qu.publish(Mailer::OUT, &MailerTask::new(to, subject, body))
                    .await?;
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Administrator {
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Http {
    pub home: String,
    pub origins: Vec<String>,
    pub port: u16,
}

impl Http {
    pub fn address(&self) -> String {
        format!("127.0.0.1:{}", self.port)
    }
}

impl Default for Http {
    fn default() -> Self {
        Self {
            home: "https://www.change-me.com".to_string(),
            port: 8080,
            origins: vec!["https://my.change-me.com".to_string()],
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
            port: 8086,
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

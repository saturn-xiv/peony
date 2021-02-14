use std::collections::HashMap;
use std::default::Default;
use std::fmt;

use flatbuffers::FlatBufferBuilder;

use super::{
    cache::redis::Config as RedisConfig,
    crypto::Key,
    errors::Result,
    mailer::Config as Mailer,
    orm::postgresql::Config as PostgreSqlConfig,
    protos::nut::{EmailTask, EmailTaskArgs},
    queue::paho::Config as MqttConfig,
    twilio::Config as TwilioConfig,
    FLAT_BUFFERS_TYPE,
};

include!(concat!(env!("OUT_DIR"), "/env.rs"));

pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const BANNER: &str = include_str!("banner.txt");

pub const LOCALHOST: &str = "127.0.0.1";

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
    pub postgresql: PostgreSqlConfig,
    pub redis: RedisConfig,
    pub mqtt: MqttConfig,
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
                let mut builder = FlatBufferBuilder::new_with_capacity(1 << 10);
                let to = builder.create_string(to);
                let subject = builder.create_string(subject);
                let body = builder.create_string(body);
                let email = EmailTask::create(
                    &mut builder,
                    &EmailTaskArgs {
                        to: Some(to),
                        subject: Some(subject),
                        body: Some(body),
                        ..Default::default()
                    },
                );
                builder.finish(email, None);
                self.mqtt
                    .publish(Mailer::OUT, FLAT_BUFFERS_TYPE, builder.finished_data())?;
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

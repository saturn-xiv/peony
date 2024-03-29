use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::http::StatusCode;
use amq_protocol_uri::{AMQPAuthority, AMQPUri, AMQPUserInfo};
use futures::StreamExt;
use lapin::{
    message::Delivery,
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties,
};
use mime::APPLICATION_JSON;
use serde::ser::Serialize;
use uuid::Uuid;

use super::super::errors::{Error, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    #[serde(rename = "virtual-host")]
    pub virtual_host: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5672,
            username: "guest".to_string(),
            password: "guest".to_string(),
            virtual_host: "/dev".to_string(),
        }
    }
}

impl Config {
    pub fn open(&self) -> RabbitMq {
        RabbitMq {
            uri: AMQPUri {
                vhost: self.virtual_host.clone(),
                authority: AMQPAuthority {
                    port: self.port,
                    host: self.host.clone(),
                    userinfo: AMQPUserInfo {
                        username: self.username.clone(),
                        password: self.password.clone(),
                    },
                },
                ..Default::default()
            },
            conn: ConnectionProperties::default(),
        }
    }
}

#[derive(Clone)]
pub struct RabbitMq {
    uri: AMQPUri,
    conn: ConnectionProperties,
}

impl RabbitMq {
    async fn open(&self, queue: &str) -> Result<Channel> {
        let con = Connection::connect_uri(self.uri.clone(), self.conn.clone()).await?;
        let ch = con.create_channel().await?;
        {
            ch.queue_declare(
                queue,
                QueueDeclareOptions {
                    exclusive: false,
                    auto_delete: false,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;
        }
        Ok(ch)
    }
}

impl RabbitMq {
    pub async fn publish<T: Serialize>(&self, queue: &str, payload: &T) -> Result<()> {
        let ch = self.open(queue).await?;
        let id = Uuid::new_v4().to_string();
        let content_type = APPLICATION_JSON.to_string();
        info!("publish task {}://{}", queue, id);
        ch.basic_publish(
            "",
            queue,
            BasicPublishOptions::default(),
            serde_json::to_vec(payload)?,
            BasicProperties::default()
                .with_message_id(id.into())
                .with_content_type(content_type.into())
                .with_timestamp(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs()),
        )
        .await?
        .await?;

        Ok(())
    }

    pub async fn consume<H: super::Handler>(
        &self,
        consumer: &str,
        queue: &str,
        handler: &H,
    ) -> Result<()> {
        let ch = self.open(queue).await?;
        let mut cm = ch
            .basic_consume(
                queue,
                consumer,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;
        info!(
            "consuming from channel {}@{}/{}...",
            consumer,
            queue,
            ch.id()
        );
        while let Some(delivery) = cm.next().await {
            handle_message(delivery?, handler).await?;
        }
        Ok(())
    }
}

async fn handle_message<H: super::Handler>(delivery: Delivery, hnd: &H) -> Result<()> {
    debug!("received message: {:?}", delivery);
    let props = &delivery.properties;
    if let Some(content_type) = props.content_type() {
        if let Some(id) = props.message_id() {
            let id = id.to_string();
            let content_type = content_type.to_string();
            info!("got message: {}[{}]", id, content_type);
            hnd.handle(&id, &content_type.parse()?, &delivery.data)?;
            delivery.ack(BasicAckOptions::default()).await?;
            return Ok(());
        }
    }

    Err(Error::Http(
        StatusCode::BAD_REQUEST,
        Some("bad message format".to_string()),
    ))
}

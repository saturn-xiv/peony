extern crate actix;
extern crate peony;
extern crate serde_json;

use std::thread::sleep;
use std::time::Duration;

use peony::{env::Config, errors::Result, parser::from_toml, queue::Handler};

struct EchoHandler {}

impl Handler for EchoHandler {
    fn handle(&self, id: &str, content_type: &str, payload: &[u8]) -> Result<()> {
        println!(
            "id: {}, content_type: {}, payload: {}",
            id,
            content_type,
            std::str::from_utf8(payload)?
        );
        sleep(Duration::from_secs(10));
        Ok(())
    }
}

async fn amqp() -> Result<()> {
    let cfg: Config = from_toml("config.toml")?;
    let qu = cfg.rabbitmq.open();
    let queue = "echo";
    for i in 1i32..10 {
        println!("publich message {}", i);
        qu.publish(
            queue,
            &format!("hello {}", i),
            "plain",
            format!("hi, {}", i).as_bytes().to_vec(),
        )
        .await?;
        sleep(Duration::from_secs(10));
    }

    qu.consume("test.rabbitmq", queue, &EchoHandler {}).await?;
    Ok(())
}

#[test]
fn test_amqp() {
    let mut ctx = actix::System::new("test-sms");
    ctx.block_on(amqp()).unwrap();
}

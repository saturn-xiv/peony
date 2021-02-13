extern crate flexbuffers;
extern crate peony;
#[macro_use]
extern crate serde_derive;

use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct Echo {
    pub message: String,
    pub id: u8,
}

const TOPIC: &'static str = "echo";
const HOST: &'static str = "192.168.8.20";

#[test]
fn test_consumer() {
    let mut cfg = peony::queue::paho::Config::default();
    cfg.host = HOST.to_string();

    println!("start consumer");
    cfg.consume(vec![TOPIC.to_string()], &|_,
                                           _,
                                           payload|
     -> peony::errors::Result<()> {
        let it: Echo = flexbuffers::from_slice(payload).unwrap();
        println!("consume message {:?}", it);
        Ok(())
    })
    .unwrap();
}

#[test]
fn test_publisher() {
    let mut cfg = peony::queue::paho::Config::default();
    cfg.host = HOST.to_string();

    println!("start publisher");
    for i in 0..std::u8::MAX {
        println!("publish message {}", i);
        cfg.publish(
            TOPIC,
            "flat",
            &flexbuffers::to_vec(&Echo {
                message: "hello, MQTT!".to_string(),
                id: i,
            })
            .unwrap(),
        )
        .unwrap();
        thread::sleep(Duration::from_secs(3));
    }
}

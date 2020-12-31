extern crate peony;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use peony::{
    env::Config,
    parser::from_toml,
    protos::{empty::Empty, nut_grpc::NutServiceClient},
};

#[test]
fn test_heartbeat() {
    let cfg: Config = from_toml("config.toml").unwrap();
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(&cfg.grpc.to_string());
    let client = NutServiceClient::new(ch);

    let reply = client.heartbeat(&Empty::default()).unwrap();
    println!("heartbeat received: {}", reply.version);
}

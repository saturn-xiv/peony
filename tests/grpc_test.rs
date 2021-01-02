extern crate peony;

use std::sync::Arc;

use actix_web::http::header::{ACCEPT_LANGUAGE, AUTHORIZATION};
use grpcio::{CallOption, ChannelBuilder, EnvBuilder, MetadataBuilder};

use peony::{
    env::Config,
    parser::from_toml,
    protos::{empty::Empty, nut_grpc::NutServiceClient},
    request::Token,
};

#[test]
fn test_heartbeat() {
    let mut mtb = MetadataBuilder::new();
    mtb.add_str(ACCEPT_LANGUAGE.as_str(), "lll")
        .unwrap()
        .add_str(
            AUTHORIZATION.as_str(),
            &format!("{}{}", Token::BEARER, "ttt"),
        )
        .unwrap();
    let headers = mtb.build();
    let option = CallOption::default().headers(headers);
    let cfg: Config = from_toml("config.toml").unwrap();
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(&cfg.grpc.to_string());
    let client = NutServiceClient::new(ch);

    let reply = client.heartbeat_opt(&Empty::default(), option).unwrap();
    println!("heartbeat received: {:?}", reply);
}

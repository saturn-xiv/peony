use std::sync::Arc;

use futures::{channel::oneshot, executor::block_on};
use grpcio::{ChannelBuilder, Environment, ResourceQuota, ServerBuilder};

use super::super::{
    env::{Config, LOCALHOST},
    errors::Result,
    plugins::{forum, nut},
    protos::{
        auth_grpc::create_user_service, forum_grpc::create_forum_service,
        nut_grpc::create_nut_service,
    },
};

pub fn launch(cfg: &Config) -> Result<()> {
    info!("start gRPC server");
    let env = Arc::new(Environment::new(cfg.grpc.threads));

    let chb = ChannelBuilder::new(env.clone())
        .set_resource_quota(ResourceQuota::new(None).resize_memory((1 << 20) * cfg.grpc.memory));

    let db = cfg.postgresql.open()?;
    let cache = cfg.redis.open()?;
    let mut server = ServerBuilder::new(env)
        .register_service(create_user_service(nut::Plugin {
            db: db.clone(),
            cache: cache.clone(),
        }))
        .register_service(create_nut_service(nut::Plugin { db, cache }))
        .register_service(create_forum_service(forum::Plugin {}))
        .bind(LOCALHOST, cfg.grpc.port)
        .channel_args(chb.build_args())
        .build()?;
    server.start();

    for (host, port) in server.bind_addrs() {
        info!("listening on {}:{}", host, port);
    }
    let (_tx, rx) = oneshot::channel::<()>();

    let _ = block_on(rx);
    let _ = block_on(server.shutdown());

    Ok(())
}

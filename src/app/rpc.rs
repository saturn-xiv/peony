use std::sync::Arc;

use futures::{channel::oneshot, executor::block_on};
use grpcio::{ChannelBuilder, Environment, ResourceQuota, ServerBuilder};

use super::super::{
    env::{Config, Context, LOCALHOST},
    errors::Result,
    plugins::{cbeta, iapt, nut},
    protos::{
        auth_grpc::create_user_service, cbeta_grpc::create_cbeta_service,
        iapt_grpc::create_iapt_service,
    },
};

pub fn launch(cfg: &Config) -> Result<()> {
    info!("start gRPC server");
    let env = Arc::new(Environment::new(cfg.grpc.threads));

    let chb = ChannelBuilder::new(env.clone())
        .set_resource_quota(ResourceQuota::new(None).resize_memory((1 << 20) * cfg.grpc.memory));

    let ctx = Arc::new(Context::new(cfg)?);
    let mut server = ServerBuilder::new(env)
        .register_service(create_cbeta_service(cbeta::Plugin { ctx: ctx.clone() }))
        .register_service(create_iapt_service(iapt::Plugin { ctx: ctx.clone() }))
        .register_service(create_user_service(nut::Plugin { ctx }))
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

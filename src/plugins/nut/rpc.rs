use futures::prelude::*;
use grpcio::{RpcContext, UnarySink};

use super::super::super::{
    env::VERSION,
    protos::{
        auth::{
            EmailRequest, ImportRequest, ResetPasswordRequest, SignInRequest, SignInResponse,
            SignUpRequest,
        },
        auth_grpc::UserService,
        empty::Empty,
        nut::{HeartbeatResponse, SetLocaleRequest},
        nut_grpc::NutService,
    },
};

impl UserService for super::Plugin {
    fn import(&mut self, _ctx: RpcContext, _req: ImportRequest, _sink: UnarySink<Empty>) {
        // TODO
    }
    fn sign_in(&mut self, _ctx: RpcContext, _req: SignInRequest, _sink: UnarySink<SignInResponse>) {
        // TODO
    }
    fn sign_up(&mut self, _ctx: RpcContext, _req: SignUpRequest, _sink: UnarySink<Empty>) {
        // TODO
    }
    fn forgot_password(&mut self, _ctx: RpcContext, _req: EmailRequest, _sink: UnarySink<Empty>) {
        // TODO
    }
    fn unlock(&mut self, _ctx: RpcContext, _req: EmailRequest, _sink: UnarySink<Empty>) {
        // TODO
    }
    fn confirm(&mut self, _ctx: RpcContext, _req: EmailRequest, _sink: UnarySink<Empty>) {
        // TODO
    }
    fn reset_password(
        &mut self,
        _ctx: RpcContext,
        _req: ResetPasswordRequest,
        _sink: UnarySink<Empty>,
    ) {
        // TODO
    }
}

impl NutService for super::Plugin {
    fn heartbeat(&mut self, ctx: RpcContext, req: Empty, sink: UnarySink<HeartbeatResponse>) {
        let mut res = HeartbeatResponse::default();
        res.version = VERSION.to_string();
        let f = sink
            .success(res)
            .map_err(move |e| error!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }

    fn set_locale(&mut self, _ctx: RpcContext, _req: SetLocaleRequest, _sink: UnarySink<Empty>) {
        // TODO
    }
}

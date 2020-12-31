use actix_web::http::header::{HeaderName, ACCEPT_LANGUAGE, AUTHORIZATION};
use futures::prelude::*;
use grpcio::{RpcContext, UnarySink};

use super::super::super::{
    env::VERSION,
    errors::Result,
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
    request::Token,
};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Session {
    token: Option<String>,
    lang: Option<String>,
}

impl Session {
    pub fn new(ctx: &RpcContext) -> Result<Self> {
        let headers = ctx.request_headers();
        let mut it = Self::default();
        for (k, v) in headers.iter() {
            let k: HeaderName = k.to_lowercase().parse()?; // HeaderName::from_lowercase()?;
            let v = std::str::from_utf8(v)?;

            match k {
                ACCEPT_LANGUAGE => {
                    it.lang = Some(v.to_string());
                }
                AUTHORIZATION => {
                    it.token = Token::parse_from_str(v);
                }
                _ => {
                    warn!("unknown header {}", k);
                }
            }
        }
        Ok(it)
    }
}

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
        let ss = Session::new(&ctx);
        debug!("{:?}", ss);
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

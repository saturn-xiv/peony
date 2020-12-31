use actix_web::http::{
    header::{HeaderName, ACCEPT_LANGUAGE, AUTHORIZATION, USER_AGENT},
    StatusCode,
};
use futures::prelude::*;
use grpcio::{RpcContext, UnarySink};

use super::super::super::{
    env::VERSION,
    errors::{Error, Result},
    jwt::Jwt,
    orm::postgresql::Connection as DbConnection,
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
    request::Token as Auth,
};
use super::{models::user::Item as User, request::CurrentUser};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub token: Option<String>,
    pub locale: String,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            token: None,
            locale: "en-US".to_string(),
        }
    }
}

impl Session {
    pub fn new(ctx: &RpcContext) -> Result<Self> {
        debug!(
            "{} {} {}",
            std::str::from_utf8(ctx.method())?,
            std::str::from_utf8(ctx.host())?,
            ctx.peer()
        );
        let headers = ctx.request_headers();
        let mut it = Self::default();
        for (k, v) in headers.iter() {
            let k: HeaderName = k.to_lowercase().parse()?;
            let v = std::str::from_utf8(v)?;

            match k {
                ACCEPT_LANGUAGE => {
                    it.locale = v.to_string();
                }
                AUTHORIZATION => {
                    it.token = Auth::parse_from_str(v);
                }
                USER_AGENT => {
                    info!("{} {}", k, v);
                }
                _ => {
                    warn!("unknown header {} {}", k, v);
                }
            }
        }
        Ok(it)
    }

    pub fn current_user(&self, db: &DbConnection, jwt: &Jwt) -> Result<User> {
        if let Some(ref token) = self.token {
            return CurrentUser::parse(token, db, jwt);
        }
        Err(Error::Http(StatusCode::NON_AUTHORITATIVE_INFORMATION, None))
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
        debug!("{:?} ", ss);
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

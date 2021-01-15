use std::ops::Deref;

use actix_web::http::{
    header::{HeaderName, ACCEPT_LANGUAGE, AUTHORIZATION, USER_AGENT},
    StatusCode,
};
use grpcio::{RpcContext, UnarySink};

use super::super::super::{
    errors::{Error, Result},
    jwt::Jwt,
    orm::postgresql::Connection as DbConnection,
    protos::{
        auth::{ImportRequest, SignInRequest, SignInResponse},
        auth_grpc::UserService,
        empty::Empty,
    },
    request::Token as Auth,
};
use super::{
    controllers::users::SignInForm,
    models::{
        policy::{Dao as PolicyDao, Role},
        user::Item as User,
    },
    request::CurrentUser,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub token: Option<String>,
    pub locale: String,
    pub peer: String,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            token: None,
            locale: "en-US".to_string(),
            peer: "nil".to_string(),
        }
    }
}

impl Session {
    pub fn new(ctx: &RpcContext) -> Self {
        Self::from_rpc_context(ctx).unwrap_or_default()
    }
    fn from_rpc_context(ctx: &RpcContext) -> Result<Self> {
        debug!(
            "{} {} {}",
            std::str::from_utf8(ctx.method())?,
            std::str::from_utf8(ctx.host())?,
            ctx.peer()
        );
        let headers = ctx.request_headers();
        let mut it = Self {
            peer: ctx.peer(),
            ..Default::default()
        };
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
    pub fn administrator(&self, db: &DbConnection, jwt: &Jwt) -> Result<User> {
        let user = self.current_user(db, jwt)?;
        if PolicyDao::can(db, user.id, &Role::Admin, &None) {
            return Ok(user);
        }
        if PolicyDao::can(db, user.id, &Role::Root, &None) {
            return Ok(user);
        }
        Err(Error::Http(StatusCode::FORBIDDEN, None))
    }
}

#[macro_export]
macro_rules! __rq_fm {
    ($r:expr,  $f:expr) => {{
        let fm: $f = $r.into();
        fm
    }};
}

impl UserService for super::Plugin {
    fn import(&mut self, _ctx: RpcContext, _req: ImportRequest, _sink: UnarySink<Empty>) {
        // TODO
    }
    fn sign_in(&mut self, ctx: RpcContext, req: SignInRequest, sink: UnarySink<SignInResponse>) {
        let ct = self.ctx.clone();
        let ss = Session::new(&ctx);
        let fm: SignInForm = req.into();
        ctx.spawn(async move {
            __unary_sink!(
                {
                    match ct.db.get() {
                        Ok(db) => {
                            let db = db.deref();
                            let it: Result<SignInResponse> =
                                fm.execute(&db, &ct.jwt, &ss.locale, &ss.peer);
                            it
                        }
                        Err(e) => {
                            let it: Result<SignInResponse> = Err(e.into());
                            it
                        }
                    }
                },
                sink
            )
        })
    }
    // fn sign_up(&mut self, ctx: RpcContext, req: SignUpRequest, sink: UnarySink<Empty>) {
    //     let ct = self.ctx.clone();
    //     let ss = Session::new(&ctx);
    //     let fm: auth::SignUp = req.into();
    //     ctx.spawn(async move { __unary_sink!(fm.execute(&ct, &ss), sink) })
    // }
    // fn forgot_password(&mut self, _ctx: RpcContext, _req: EmailRequest, _sink: UnarySink<Empty>) {
    //     // TODO
    // }
    // fn unlock(&mut self, _ctx: RpcContext, _req: EmailRequest, _sink: UnarySink<Empty>) {
    //     // TODO
    // }
    // fn confirm(&mut self, _ctx: RpcContext, _req: EmailRequest, _sink: UnarySink<Empty>) {
    //     // TODO
    // }
    // fn reset_password(
    //     &mut self,
    //     _ctx: RpcContext,
    //     _req: ResetPasswordRequest,
    //     _sink: UnarySink<Empty>,
    // ) {
    //     // TODO
    // }
}

// impl NutService for super::Plugin {
//     fn heartbeat(&mut self, ctx: RpcContext, _req: Empty, sink: UnarySink<HeartbeatResponse>) {
//         let ss = Session::new(&ctx);
//         let ct = self.ctx.clone();
//         ctx.spawn(async move { __unary_sink!(site::Heartbeat::execute(&ct, &ss), sink) })
//     }

//     fn set_locale(&mut self, _ctx: RpcContext, _req: SetLocaleRequest, _sink: UnarySink<Empty>) {
//         // TODO
//     }
// }

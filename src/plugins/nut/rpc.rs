use std::ops::Deref;

use actix_web::http::{
    header::{HeaderName, ACCEPT_LANGUAGE, AUTHORIZATION, USER_AGENT},
    StatusCode,
};
use grpcio::{RpcContext, UnarySink};

use super::super::super::{
    cache::{redis::Pool as CachePool, Provider as CacheProvider},
    env::VERSION,
    errors::{Error, Result},
    jwt::Jwt,
    orm::{
        migration::Dao as MigrationDao,
        postgresql::{Connection as DbConnection, Pool as DbPool},
    },
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

impl super::Plugin {
    fn _heartbeat(db: DbPool, ch: CachePool) -> Result<HeartbeatResponse> {
        let db = db.get()?;
        let db = db.deref();

        Ok(HeartbeatResponse {
            version: VERSION.to_string(),
            postgresql: MigrationDao::version(db)?,
            redis: ch.version()?,
            ..Default::default()
        })
    }
}

impl NutService for super::Plugin {
    fn heartbeat(&mut self, ctx: RpcContext, req: Empty, sink: UnarySink<HeartbeatResponse>) {
        let ss = Session::new(&ctx);
        debug!("{:?} {:?} ", ss, req);
        let db = self.db.clone();
        let ch = self.cache.clone();

        let f = async move {
            __unary_sink!(Self::_heartbeat(db, ch), sink)
            // match Self::_heartbeat(db, ch) {
            //     Ok(v) => {
            //         debug!("{:?}", v);
            //         sink.success(v);
            //     }
            //     Err(e) => {
            //         error!("{:?}", e);
            //         sink.fail(e.into());
            //     }
            // }
        };
        ctx.spawn(f)
        // let f = async move {
        //     let it = Self::_heartbeat(db, ch);
        //     match it {
        //         Ok(v) => {
        //             sink.success(v);
        //             Ok(())
        //         }
        //         Err(e) => {
        //             sink.fail(e.to_rpc_status());
        //             Err(e)
        //         }
        //     }
        // }
        // .map_err(|e: Error| {
        //     error!("failed to reply {:?}", e);
        // })
        // .map(|_| ());
    }

    fn set_locale(&mut self, _ctx: RpcContext, _req: SetLocaleRequest, _sink: UnarySink<Empty>) {
        // TODO
    }
}

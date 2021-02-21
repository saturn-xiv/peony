pub mod attachements;
pub mod network;
pub mod ntp;
pub mod users;
pub mod vpn;

use std::fmt;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::result::Result as StdResult;
use std::sync::Arc;
use std::time::Duration;

use actix_web::{
    dev::Payload, error::Error as ActixError, get, http::StatusCode, post, web, FromRequest,
    HttpRequest, HttpResponse, Responder,
};
use futures::future::Future;
use nix::sys::reboot::RebootMode;

use super::super::super::{
    cache::{redis::Pool as DbPool, KV},
    errors::{Error, Result},
    jwt::Jwt,
    request::Token as Auth,
};
use super::super::nut::request::{Action, Token};

pub fn mount(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(reboot)
            .service(reset)
            .service(token)
            .service(web::scope("/vpn").service(vpn::get).service(vpn::set))
            .service(web::scope("/ntp").service(ntp::get).service(ntp::set))
            .service(
                web::scope("/attachments")
                    .service(attachements::index)
                    .service(attachements::create)
                    .service(attachements::show)
                    .service(attachements::update)
                    .service(attachements::destory),
            )
            .service(
                web::scope("/network")
                    .service(network::status)
                    .service(
                        web::scope("/wifi")
                            .service(network::wifi::get)
                            .service(network::wifi::set),
                    )
                    .service(
                        web::scope("/ether")
                            .service(network::ether::get)
                            .service(network::ether::set),
                    ),
            )
            .service(
                web::scope("/users")
                    .service(users::sign_in)
                    .service(users::logs)
                    .service(users::get_profile)
                    .service(users::set_profile)
                    .service(users::change_password),
            ),
    );
}

#[post("/reboot")]
async fn reboot(_user: CurrentUser) -> Result<impl Responder> {
    actix_rt::time::delay_for(Duration::from_secs(5)).await;
    warn!("prepare to reboot");
    nix::sys::reboot::reboot(RebootMode::RB_AUTOBOOT)?;
    Ok(HttpResponse::Ok().json(()))
}

#[post("/reset")]
async fn reset(_user: CurrentUser, db: web::Data<DbPool>) -> Result<impl Responder> {
    warn!("prepare to reset");
    let mut db = db.get()?;
    let db = db.deref_mut();
    let _: String = redis::cmd("flushall").query(db)?;
    actix_rt::time::delay_for(Duration::from_secs(5)).await;
    warn!("reboot");
    nix::sys::reboot::reboot(RebootMode::RB_AUTOBOOT)?;
    Ok(HttpResponse::Ok().json(()))
}

#[get("/token")]
async fn token(_user: CurrentUser) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub password: String,
}

impl User {
    pub const ID: &'static str = "e883c3ef-fe83-43d5-94dc-5658c7e733ac";
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentUser(pub String);

impl fmt::Display for CurrentUser {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl FromRequest for CurrentUser {
    type Config = ();
    type Error = ActixError;
    type Future = Pin<Box<dyn Future<Output = StdResult<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let auth = Auth::from_request(req, pl);
        let jwt = web::Data::<Arc<Jwt>>::from_request(req, pl);
        let db = web::Data::<DbPool>::from_request(req, pl);

        Box::pin(async move {
            let auth = auth.await?;
            let jwt = jwt.await?;
            let db = db.await?;
            let db = db.deref();
            let db = db.deref();

            if let Ok(auth) = jwt.parse::<Token>(&auth.0) {
                if auth.claims.act == Action::SignIn {
                    if let Ok(user) = KV::get::<String, User>(db, &User::ID.to_string()) {
                        return Ok(Self(user.name));
                    }
                }
            }
            Err(Error::Http(StatusCode::FORBIDDEN, None).into())
        })
    }
}

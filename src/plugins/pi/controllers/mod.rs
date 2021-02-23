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
use chrono::{Datelike, Utc};
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
                    .service(network::ping)
                    .service(network::get)
                    .service(network::set),
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

#[get("/token/{year}")]
async fn token(
    _user: CurrentUser,
    db: web::Data<DbPool>,
    jwt: web::Data<Jwt>,
    years: web::Path<i32>,
) -> Result<impl Responder> {
    let db = db.deref();
    let db = db.deref();
    let user: User = KV::get(db, &User::KEY)?;
    let jwt = jwt.deref();
    let it = user.token(jwt, years.0)?;
    Ok(HttpResponse::Ok().json(it))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub password: String,
}

impl User {
    pub const KEY: &'static str = "administrator";
    pub const ID: &'static str = "e883c3ef-fe83-43d5-94dc-5658c7e733ac";

    pub fn token(&self, jwt: &Jwt, years: i32) -> Result<String> {
        let nbf = Utc::now().naive_utc();
        if let Some(exp) = nbf.with_year(nbf.year() + years) {
            let it = jwt.sum(
                None,
                &Token {
                    sub: self.name.clone(),
                    uid: Self::ID.to_string(),
                    act: Action::SignIn,
                    nbf: nbf.timestamp(),
                    exp: exp.timestamp(),
                },
            )?;
            return Ok(it);
        }
        Err(Error::Http(StatusCode::BAD_REQUEST, None))
    }
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

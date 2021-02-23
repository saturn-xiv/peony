use std::ops::{Deref, DerefMut};

use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use chrono::{NaiveDateTime, Utc};
use redis_::Connection as Db;
use validator::Validate;

use super::super::super::super::{
    cache::{redis::Pool as DbPool, KV},
    errors::{Error, Result},
    jwt::Jwt,
};
use super::{CurrentUser, User};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    created_at: NaiveDateTime,
    message: String,
}

impl Log {
    const KEY: &'static str = "logs";
    pub fn list(db: &mut Db) -> Result<Vec<Self>> {
        let mut items = Vec::new();
        for f in redis::cmd("hkeys")
            .arg(Self::KEY)
            .query::<Vec<String>>(db)?
        {
            items.push(Log {
                created_at: f.parse()?,
                message: redis::cmd("hget").arg(Self::KEY).arg(f).query(db)?,
            });
        }
        Ok(items)
    }
    pub fn create(db: &mut Db, message: &str) -> Result<()> {
        let now = Utc::now().naive_local();
        let _: String = redis::cmd("hset")
            .arg("logs")
            .arg(now.to_string())
            .arg(message)
            .query(db)?;
        Ok(())
    }
}

#[post("/users/sign-in")]
async fn sign_in(
    form: web::Json<User>,
    jwt: web::Data<Jwt>,
    db: web::Data<DbPool>,
) -> Result<impl Responder> {
    let db = db.deref();
    let db = db.deref();
    let form = form.deref();
    form.validate()?;
    let user: User = KV::get(db, &User::KEY.to_string()).unwrap_or_default();

    let mut db = db.get()?;
    let db = db.deref_mut();
    if *form != user {
        Log::create(db, "Sign in failed: bad username & password.")?;
        return Err(Error::Http(StatusCode::FORBIDDEN, None));
    }
    Log::create(db, "Sign in successed.")?;
    let jwt = jwt.deref();
    let it = user.token(jwt, 1)?;
    Ok(HttpResponse::Ok().json(it))
}

#[get("/users/logs")]
async fn logs(_user: CurrentUser, db: web::Data<DbPool>) -> Result<impl Responder> {
    let mut db = db.get()?;
    let db = db.deref_mut();
    let items = Log::list(db)?;
    Ok(HttpResponse::Ok().json(items))
}

#[derive(Validate, Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub current_password: String,
    #[validate(length(min = 6))]
    pub new_password: String,
}

#[post("/users/profile")]
async fn set_profile(
    _user: CurrentUser,
    form: web::Json<Profile>,
    db: web::Data<DbPool>,
) -> Result<impl Responder> {
    let db = db.deref();
    let db = db.deref();
    let form = form.deref();
    form.validate()?;
    let user: User = KV::get(db, &User::KEY.to_string()).unwrap_or_default();
    if form.current_password != user.password {
        return Err(Error::Http(StatusCode::FORBIDDEN, None));
    }
    KV::set(
        db,
        &User::KEY.to_string(),
        &User {
            name: form.name.clone(),
            password: form.new_password.clone(),
        },
    )?;
    Ok(HttpResponse::Ok().json(()))
}

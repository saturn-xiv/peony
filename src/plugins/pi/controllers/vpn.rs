use std::fs::{remove_file, OpenOptions};
use std::io::prelude::*;
use std::ops::Deref;
use std::os::unix::fs::OpenOptionsExt;
use std::path::{Component, Path};

use actix_web::{get, post, web, HttpResponse, Responder};
use validator::Validate;

use super::super::super::super::{
    cache::{redis::Pool as DbPool, Kv},
    errors::Result,
};
use super::CurrentUser;

#[derive(Validate, Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    pub enable: bool,
    #[validate(length(min = 1))]
    pub body: String,
}

impl Form {
    pub const KEY: &'static str = "openvpn";

    pub fn save(&self) -> Result<()> {
        let file = Path::new(&Component::RootDir)
            .join("etc")
            .join("openvpn")
            .join("client.conf");
        if self.enable {
            info!("generate file {}", file.display());
            let mut fd = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .mode(0o600)
                .open(file)?;
            fd.write_all(self.body.as_bytes())?;
        } else if file.exists() {
            info!("delete file {}", file.display());
            remove_file(file)?;
        }
        Ok(())
    }
}

#[get("/")]
pub async fn get(_user: CurrentUser, db: web::Data<DbPool>) -> Result<impl Responder> {
    let db = db.deref();
    let db = db.deref();
    let it: Form = Kv::get(db, &Form::KEY.to_string()).unwrap_or_default();
    Ok(HttpResponse::Ok().json(it))
}

#[post("/")]
pub async fn set(
    _user: CurrentUser,
    form: web::Json<Form>,
    db: web::Data<DbPool>,
) -> Result<impl Responder> {
    let db = db.deref();
    let db = db.deref();
    let form = form.deref();
    form.validate()?;
    form.save()?;
    Kv::set(db, &Form::KEY.to_string(), form)?;
    Ok(HttpResponse::Ok().json(()))
}

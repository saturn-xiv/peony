pub mod ether;
pub mod wifi;

use actix_web::{get, web, HttpResponse, Responder};

use super::super::super::super::{cache::redis::Pool as Db, errors::Result};

#[get("/status")]
async fn status(db: web::Data<Db>) -> Result<impl Responder> {
    let _db = db.get()?;
    // TODO
    Ok(HttpResponse::Ok().json(()))
}

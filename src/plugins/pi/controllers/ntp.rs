use actix_web::{get, post, HttpResponse, Responder};

use super::super::super::super::errors::Result;

#[get("/")]
async fn get() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}

#[post("/")]
async fn set() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}

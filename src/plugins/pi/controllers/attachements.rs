use actix_web::{delete, get, post, web, HttpResponse, Responder};

use super::super::super::super::errors::Result;

#[get("/")]
async fn index() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}

#[post("/")]
async fn create() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}

#[get("/{id}")]
async fn show(_path: web::Path<(String,)>) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}

#[post("/{id}")]
async fn update(_path: web::Path<(String,)>) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}

#[delete("/{id}")]
async fn destory(_path: web::Path<(String,)>) -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}

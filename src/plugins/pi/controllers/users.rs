use actix_web::{get, post, HttpResponse, Responder};

use super::super::super::super::errors::Result;

#[post("/users/sign-in")]
async fn sign_in() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}

#[get("/users/logs")]
async fn logs() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}

#[get("/users/profile")]
async fn get_profile() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}

#[post("/users/profile")]
async fn set_profile() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}

#[post("/users/change-password")]
async fn change_password() -> Result<impl Responder> {
    Ok(HttpResponse::Ok().json(()))
}

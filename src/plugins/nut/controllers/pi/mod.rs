use std::time::Duration;

use actix_web::{get, HttpResponse, Responder};

use super::super::super::super::errors::Result;

#[get("/reboot")]
async fn reboot() -> Result<impl Responder> {
    actix_rt::time::delay_for(Duration::from_secs(5)).await;
    // TODO
    Ok(HttpResponse::Ok().json(()))
}

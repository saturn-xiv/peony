use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use actix_session::CookieSession;
use actix_web::{cookie::SameSite, middleware, web, App, HttpServer};

use super::super::{
    crypto::{Aes, Hmac},
    env::{Config, Environment},
    errors::Result,
    jwt::Jwt,
    plugins::nut,
};

pub async fn launch(cfg: Config) -> Result<()> {
    info!("start http server");
    let aes = web::Data::new(Aes::new(&cfg.secrets.0)?);
    let hmac = web::Data::new(Hmac::new(&cfg.secrets.0)?);
    let jwt = web::Data::new(Jwt::new(cfg.secrets.0.clone()));
    let db = cfg.postgresql.open()?;
    let cache = cfg.redis.open()?;
    let cfg = web::Data::new(cfg);

    {
        info!("start worker thread");
        let cfg = Arc::new(cfg.clone());
        actix_rt::spawn(async move {
            let mut timeout = actix_rt::time::interval(Duration::from_secs(5));
            let cfg = cfg.deref();
            loop {
                timeout.tick().await;
                if let Err(e) = worker(cfg).await {
                    error!("worker thread panic {:?}", e);
                }
            }
        });
    }

    let address = cfg.http.address();
    let workers = cfg.http.workers;
    let key: Result<Vec<u8>> = cfg.secrets.clone().into();
    let key = key?;
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                CookieSession::signed(&key)
                    .name("s")
                    .path("/")
                    .http_only(true)
                    .same_site(SameSite::Strict)
                    .expires_in(60 * 60 * 24)
                    .secure(cfg.env == Environment::Production),
            )
            .app_data(cfg.clone())
            .app_data(jwt.clone())
            .app_data(aes.clone())
            .app_data(hmac.clone())
            .data(db.clone())
            .data(cache.clone())
            .configure(nut::controllers::mount)
    })
    .bind(address)?
    .workers(workers)
    .run()
    .await?;
    Ok(())
}

async fn worker(_cfg: &Config) -> Result<()> {
    debug!("process queue job");
    // TODO
    Ok(())
}

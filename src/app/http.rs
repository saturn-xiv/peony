use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;

use actix_web::{middleware, web, App, HttpServer};

use super::super::{
    crypto::{Aes, Hmac},
    env::Config,
    errors::Result,
    jwt::Jwt,
    plugins::nut,
};

pub async fn launch(cfg: Config) -> Result<()> {
    info!("start http server");
    let address = cfg.http.address();
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

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(cfg.clone())
            .app_data(jwt.clone())
            .app_data(aes.clone())
            .app_data(hmac.clone())
            .data(db.clone())
            .data(cache.clone())
            .configure(nut::controllers::mount)
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}

async fn worker(_cfg: &Config) -> Result<()> {
    debug!("process queue job");
    // TODO
    Ok(())
}

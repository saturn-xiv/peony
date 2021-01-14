pub mod models;
pub mod schema;

use std::ops::Deref;
use std::sync::Arc;

use actix_web::http::StatusCode;

use super::super::super::{
    env::Context,
    errors::{Error, Result},
    orm::migration::New as Migration,
    request::https_client,
};

#[derive(Clone)]
pub struct Plugin {
    pub ctx: Arc<Context>,
}

impl super::super::Plugin for Plugin {
    fn migrations<'a>() -> Vec<Migration<'a>> {
        let mut items = Vec::new();
        items.push(Migration {
            version: "20210113142934",
            name: "create-crawler",
            up: include_str!("up.sql"),
            down: include_str!("down.sql"),
        });
        items
    }
}

impl Plugin {
    pub async fn pull(&self, name: &str, url: &str) -> Result<()> {
        use self::models::log::Dao as LogDao;
        debug!("fetch {}", url);

        let mut res = https_client()?.finish().get(url).send().await?;
        let body = res.body().await?;
        let body = std::str::from_utf8(&body)?;
        match res.status() {
            StatusCode::OK => {
                let db = self.ctx.db.get()?;
                let db = db.deref();
                LogDao::create(db, name, url, &body)?;
                Ok(())
            }
            v => Err(Error::Http(v, Some(body.to_string()))),
        }
    }
}

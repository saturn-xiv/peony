pub mod models;
pub mod schema;

use std::ops::Deref;
use std::sync::Arc;

use reqwest::{Client, StatusCode};

use super::super::super::{
    env::Context,
    errors::{Error, Result},
    orm::migration::New as Migration,
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

        let res = Client::new().get(url).send().await?;
        let body = match res.status() {
            StatusCode::OK => {
                let it = res.text().await?;
                Ok(it)
            }
            v => Err(Error::Http(v, Some(res.text().await?))),
        };

        let db = self.ctx.db.get()?;
        let db = db.deref();
        LogDao::create(db, name, url, &body?)?;
        Ok(())
    }
}

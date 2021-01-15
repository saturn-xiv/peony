use std::ops::Deref;

use super::super::super::super::super::{
    cache::redis::Pool as Cache,
    cache::Provider as CacheProvider,
    env::{Context, VERSION},
    errors::Result,
    orm::{migration::Dao as MigrationDao, postgresql::Pool as Db},
};

pub struct Heartbeat;

impl Heartbeat {
    // pub fn execute(db: &Db, ss: &Session) -> Result<HeartbeatResponse> {
    //     let db = ctx.db.get()?;
    //     let db = db.deref();
    //     ss.administrator(db, &ctx.jwt)?;

    //     Ok(HeartbeatResponse {
    //         version: VERSION.to_string(),
    //         postgresql: MigrationDao::version(db)?,
    //         redis: ctx.cache.version()?,
    //         ..Default::default()
    //     })
    // }
}

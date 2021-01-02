use std::ops::Deref;

use super::super::super::super::{
    cache::Provider as CacheProvider,
    env::{Context, VERSION},
    errors::Result,
    orm::migration::Dao as MigrationDao,
    protos::nut::HeartbeatResponse,
};
use super::Session;

pub struct Heartbeat {}

impl Heartbeat {
    pub fn execute(ctx: &Context, ss: &Session) -> Result<HeartbeatResponse> {
        let db = ctx.db.get()?;
        let db = db.deref();
        ss.administrator(db, &ctx.jwt)?;

        Ok(HeartbeatResponse {
            version: VERSION.to_string(),
            postgresql: MigrationDao::version(db)?,
            redis: ctx.cache.version()?,
            ..Default::default()
        })
    }
}

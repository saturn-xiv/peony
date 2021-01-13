use chrono::NaiveDateTime;
use diesel::{
    delete,
    dsl::{now, IntervalDsl},
    insert_into,
    prelude::*,
};

use super::super::super::super::super::{errors::Result, orm::postgresql::Connection};
use super::super::schema::ops_crawler_logs;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub body: String,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn latest(&self, name: &str) -> Result<Item>;
    fn create(&self, name: &str, url: &str, body: &str) -> Result<()>;
    fn delete(&self, years: i32) -> Result<()>;
}

impl Dao for Connection {
    fn latest(&self, name: &str) -> Result<Item> {
        let it = ops_crawler_logs::dsl::ops_crawler_logs
            .filter(ops_crawler_logs::dsl::name.eq(name))
            .order(ops_crawler_logs::dsl::created_at.desc())
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(&self, name: &str, url: &str, body: &str) -> Result<()> {
        insert_into(ops_crawler_logs::dsl::ops_crawler_logs)
            .values((
                ops_crawler_logs::dsl::name.eq(name),
                ops_crawler_logs::dsl::url.eq(url),
                ops_crawler_logs::dsl::body.eq(body),
            ))
            .execute(self)?;
        Ok(())
    }

    fn delete(&self, years: i32) -> Result<()> {
        delete(
            ops_crawler_logs::dsl::ops_crawler_logs
                .filter(ops_crawler_logs::dsl::created_at.lt(now - years.years())),
        )
        .execute(self)?;
        Ok(())
    }
}

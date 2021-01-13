use std::fmt;

use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};

use super::super::super::super::super::{errors::Result, orm::postgresql::Connection};
use super::super::schema::ops_cron_tasks;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub schedule: String,
    pub user: Option<String>,
    pub command: String,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn by_miniutes(v: usize) -> String {
        format!("0/{} * * * *", v)
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.schedule,
            self.user.as_deref().unwrap_or("root"),
            self.command
        )
    }
}

pub trait Dao {
    fn by_id(&self, id: i64) -> Result<Item>;
    fn create(&self, schedule: &str, user: &Option<String>, command: &str) -> Result<()>;
    fn update(&self, id: i64, schedule: &str, user: &Option<String>, command: &str) -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn delete(&self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&self, id: i64) -> Result<Item> {
        let it = ops_cron_tasks::dsl::ops_cron_tasks
            .filter(ops_cron_tasks::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(&self, schedule: &str, user: &Option<String>, command: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(ops_cron_tasks::dsl::ops_cron_tasks)
            .values((
                ops_cron_tasks::dsl::schedule.eq(schedule),
                ops_cron_tasks::dsl::user.eq(user),
                ops_cron_tasks::dsl::command.eq(command),
                ops_cron_tasks::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn update(&self, id: i64, schedule: &str, user: &Option<String>, command: &str) -> Result<()> {
        let now = Utc::now().naive_utc();
        update(ops_cron_tasks::dsl::ops_cron_tasks.filter(ops_cron_tasks::dsl::id.eq(id)))
            .set((
                ops_cron_tasks::dsl::schedule.eq(schedule),
                ops_cron_tasks::dsl::user.eq(user),
                ops_cron_tasks::dsl::command.eq(command),
                ops_cron_tasks::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn all(&self) -> Result<Vec<Item>> {
        let items = ops_cron_tasks::dsl::ops_cron_tasks
            .order(ops_cron_tasks::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn delete(&self, id: i64) -> Result<()> {
        delete(ops_cron_tasks::dsl::ops_cron_tasks.filter(ops_cron_tasks::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
}

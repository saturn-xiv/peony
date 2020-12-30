use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use mime::Mime;

use super::super::super::super::{errors::Result, orm::postgresql::Connection};
use super::super::schema::attachments;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub size: i64,
    pub content_type: String,
    pub url: String,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn by_id(&self, id: i64) -> Result<Item>;
    fn create(
        &self,
        user: i64,
        title: &str,
        content_type: &Mime,
        url: &str,
        size: i64,
    ) -> Result<()>;
    fn update(&self, id: i64, title: &str, content_type: &Mime, url: &str, size: i64)
        -> Result<()>;
    fn all(&self) -> Result<Vec<Item>>;
    fn by_user(&self, user: i64) -> Result<Vec<Item>>;
    fn delete(&self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn by_id(&self, id: i64) -> Result<Item> {
        let it = attachments::dsl::attachments
            .filter(attachments::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn create(
        &self,
        user: i64,
        title: &str,
        content_type: &Mime,
        url: &str,
        size: i64,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let content_type = content_type.to_string();
        insert_into(attachments::dsl::attachments)
            .values((
                attachments::dsl::user_id.eq(user),
                attachments::dsl::title.eq(title),
                attachments::dsl::content_type.eq(content_type),
                attachments::dsl::url.eq(url),
                attachments::dsl::size.eq(size),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn update(
        &self,
        id: i64,
        title: &str,
        content_type: &Mime,
        url: &str,
        size: i64,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let content_type = content_type.to_string();
        update(attachments::dsl::attachments.filter(attachments::dsl::id.eq(id)))
            .set((
                attachments::dsl::title.eq(title),
                attachments::dsl::content_type.eq(content_type),
                attachments::dsl::url.eq(url),
                attachments::dsl::size.eq(size),
                attachments::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn all(&self) -> Result<Vec<Item>> {
        let items = attachments::dsl::attachments
            .order(attachments::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn by_user(&self, user: i64) -> Result<Vec<Item>> {
        let items = attachments::dsl::attachments
            .filter(attachments::dsl::user_id.eq(user))
            .order(attachments::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }

    fn delete(&self, id: i64) -> Result<()> {
        delete(attachments::dsl::attachments.filter(attachments::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}

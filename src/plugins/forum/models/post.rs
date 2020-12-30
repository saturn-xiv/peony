use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use mime::Mime;

use super::super::super::super::{errors::Result, orm::postgresql::Connection};
use super::super::schema::forum_posts;

#[derive(Queryable, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub topic_id: i64,
    pub post_id: Option<i64>,
    pub body: String,
    pub media_type: String,
    pub version: i64,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add(
        &self,
        user: i64,
        topic: i64,
        post: Option<i64>,
        body: &str,
        media_type: &Mime,
    ) -> Result<()>;
    fn get(&self, id: i64) -> Result<Item>;
    fn update(&self, id: i64, body: &str, media_type: &Mime) -> Result<()>;
    fn latest(&self) -> Result<Vec<Item>>;
    fn by_user(&self, id: i64) -> Result<Vec<Item>>;
    fn by_topic(&self, id: i64) -> Result<Vec<Item>>;
    fn delete(&self, id: i64) -> Result<()>;
}

impl Dao for Connection {
    fn add(
        &self,
        user: i64,
        topic: i64,
        post: Option<i64>,
        body: &str,
        media_type: &Mime,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        insert_into(forum_posts::dsl::forum_posts)
            .values((
                forum_posts::dsl::user_id.eq(user),
                forum_posts::dsl::topic_id.eq(topic),
                forum_posts::dsl::post_id.eq(post),
                forum_posts::dsl::body.eq(body),
                forum_posts::dsl::media_type.eq(&media_type.to_string()),
                forum_posts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }
    fn get(&self, id: i64) -> Result<Item> {
        let it = forum_posts::dsl::forum_posts
            .filter(forum_posts::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn update(&self, id: i64, body: &str, media_type: &Mime) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = forum_posts::dsl::forum_posts.filter(forum_posts::dsl::id.eq(id));
        update(it)
            .set((
                forum_posts::dsl::body.eq(body),
                forum_posts::dsl::media_type.eq(&media_type.to_string()),
                forum_posts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;

        Ok(())
    }
    fn latest(&self) -> Result<Vec<Item>> {
        let items = forum_posts::dsl::forum_posts
            .order(forum_posts::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_user(&self, id: i64) -> Result<Vec<Item>> {
        let items = forum_posts::dsl::forum_posts
            .filter(forum_posts::dsl::user_id.eq(id))
            .order(forum_posts::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn by_topic(&self, id: i64) -> Result<Vec<Item>> {
        let items = forum_posts::dsl::forum_posts
            .filter(forum_posts::dsl::topic_id.eq(id))
            .order(forum_posts::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn delete(&self, id: i64) -> Result<()> {
        delete(forum_posts::dsl::forum_posts.filter(forum_posts::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}

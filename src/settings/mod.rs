pub mod schema;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*, update};
use serde::{de::DeserializeOwned, ser::Serialize};

use super::{crypto::Secret, errors::Result, orm::postgresql::Connection};

use self::schema::settings;

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub key: String,
    pub value: Vec<u8>,
    pub salt: Option<Vec<u8>>,
    pub version: i64,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "settings"]
pub struct New<'a> {
    pub key: &'a str,
    pub value: &'a [u8],
    pub salt: Option<&'a [u8]>,
    pub updated_at: &'a NaiveDateTime,
}

pub trait Dao {
    fn get<V: DeserializeOwned, E: Secret>(&self, e: &E, key: &str) -> Result<V>;
    fn set<V: Serialize, E: Secret>(&self, e: &E, k: &str, v: &V, f: bool) -> Result<()>;
}

impl Dao for Connection {
    fn get<V: DeserializeOwned, E: Secret>(&self, e: &E, k: &str) -> Result<V> {
        let it = settings::dsl::settings
            .filter(settings::dsl::key.eq(k))
            .first::<Item>(self)?;

        let val = match it.salt {
            Some(salt) => e.decrypt(&it.value, &salt)?,
            None => it.value,
        };
        Ok(flexbuffers::from_slice(val.as_slice())?)
    }

    fn set<V: Serialize, E: Secret>(&self, e: &E, k: &str, v: &V, f: bool) -> Result<()> {
        let buf = flexbuffers::to_vec(v)?;

        let (val, salt) = if f {
            let (val, salt) = e.encrypt(&buf)?;
            (val, Some(salt))
        } else {
            (buf, None)
        };

        let now = Utc::now().naive_utc();

        match settings::dsl::settings
            .filter(settings::dsl::key.eq(k))
            .first::<Item>(self)
        {
            Ok(it) => {
                let it = settings::dsl::settings.filter(settings::dsl::id.eq(&it.id));
                update(it)
                    .set((
                        settings::dsl::value.eq(&val),
                        settings::dsl::salt.eq(&salt),
                        settings::dsl::updated_at.eq(&now),
                    ))
                    .execute(self)?;
            }
            Err(_) => {
                insert_into(settings::dsl::settings)
                    .values(&New {
                        key: k,
                        value: &val,
                        salt: salt.as_ref().map(|x| x as _),
                        updated_at: &now,
                    })
                    .execute(self)?;
            }
        };
        Ok(())
    }
}

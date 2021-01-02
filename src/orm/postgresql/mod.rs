pub mod schema;

use std::default::Default;
use std::fmt;

use chrono::Utc;
use diesel::{connection::SimpleConnection, delete, insert_into, prelude::*, sql_query, update};

use super::super::errors::Result;
use super::migration::{Dao, Item, New, Table, Version};

use self::schema::schema_migrations;

// https://www.postgresql.org/docs/current/runtime-config-logging.html
// /var/lib/postgres/data/postgresql.conf: log_statement = 'all'
// journalctl -f -u postgresql
pub type Connection = diesel::pg::PgConnection;
pub type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<Connection>>;
pub type PooledConnection =
    diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<Connection>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub name: String,
    pub user: String,
    pub password: Option<String>,
}

impl Config {
    pub fn open(&self) -> Result<Pool> {
        let manager = diesel::r2d2::ConnectionManager::<Connection>::new(&self.to_string());
        Ok(Pool::new(manager)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5432,
            user: "postgres".to_string(),
            name: "dev".to_string(),
            password: None,
        }
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password.as_deref().unwrap_or(""),
            self.host,
            self.port,
            self.name
        )
    }
}

impl Dao for Connection {
    fn check(&self) -> Result<()> {
        let rst = sql_query("SELECT table_name AS name FROM information_schema.tables WHERE table_name = 'schema_migrations'").load::<Table>(self)?;
        if rst.is_empty() {
            info!("database is empty");
            self.batch_execute(include_str!("up.sql"))?;
        }
        Ok(())
    }
    fn load(&self, items: &[New]) -> Result<()> {
        for it in items {
            info!("find migration: {}", it);
            let c: i64 = schema_migrations::dsl::schema_migrations
                .filter(schema_migrations::dsl::version.eq(it.version))
                .filter(schema_migrations::dsl::name.eq(it.name))
                .count()
                .get_result(self)?;
            if c == 0 {
                info!("migration {} not exist, insert it", it);
                insert_into(schema_migrations::dsl::schema_migrations)
                    .values((
                        schema_migrations::dsl::version.eq(it.version),
                        schema_migrations::dsl::name.eq(it.name),
                        schema_migrations::dsl::up.eq(it.up),
                        schema_migrations::dsl::down.eq(it.down),
                    ))
                    .execute(self)?;
            }
        }

        Ok(())
    }
    fn migrate(&self) -> Result<()> {
        let now = Utc::now().naive_utc();
        for it in schema_migrations::dsl::schema_migrations
            .filter(schema_migrations::dsl::run_at.is_null())
            .order(schema_migrations::dsl::version.asc())
            .load::<Item>(self)?
        {
            info!("run migrate {}", it.up);
            self.batch_execute(&it.up)?;

            let it = schema_migrations::dsl::schema_migrations
                .filter(schema_migrations::dsl::id.eq(&it.id));
            update(it)
                .set(schema_migrations::dsl::run_at.eq(&now))
                .execute(self)?;
        }

        Ok(())
    }
    fn rollback(&self) -> Result<()> {
        match schema_migrations::dsl::schema_migrations
            .filter(schema_migrations::dsl::run_at.is_not_null())
            .order(schema_migrations::dsl::version.desc())
            .first::<Item>(self)
        {
            Ok(it) => {
                info!("rollback {}", it.down);
                self.batch_execute(&it.down)?;
                delete(
                    schema_migrations::dsl::schema_migrations
                        .filter(schema_migrations::dsl::id.eq(it.id)),
                )
                .execute(self)?;
            }
            Err(_) => warn!("database is empty"),
        };

        Ok(())
    }
    fn status(&self) -> Result<Vec<Item>> {
        let items = schema_migrations::dsl::schema_migrations
            .order(schema_migrations::dsl::version.asc())
            .load(self)?;
        Ok(items)
    }
    fn version(&self) -> Result<String> {
        let it: Version = sql_query("SELECT VERSION() AS value").get_result(self)?;
        Ok(it.value)
    }
}

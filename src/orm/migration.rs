use std::cmp::Ordering;
use std::fmt;

use chrono::NaiveDateTime;
use diesel::sql_types::Text;

use super::super::errors::Result;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub version: String,
    pub name: String,
    pub up: String,
    pub down: String,
    pub run_at: Option<NaiveDateTime>,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:<14} {:<32} {}",
            self.version,
            self.name,
            match self.run_at {
                Some(v) => v.to_string(),
                None => "N/A".to_string(),
            },
        )
    }
}

#[derive(Eq, Clone)]
pub struct New<'a> {
    pub version: &'a str,
    pub name: &'a str,
    pub up: &'a str,
    pub down: &'a str,
}

impl<'a> fmt::Display for New<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{}", self.version, self.name)
    }
}

impl<'a> Ord for New<'a> {
    fn cmp(&self, other: &New) -> Ordering {
        self.version.cmp(&other.version)
    }
}

impl<'a> PartialOrd for New<'a> {
    fn partial_cmp(&self, other: &New) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for New<'a> {
    fn eq(&self, other: &New) -> bool {
        self.version == other.version
    }
}

pub trait Dao {
    fn load(&self, items: &[New]) -> Result<()>;
    fn migrate(&self) -> Result<()>;
    fn rollback(&self) -> Result<()>;
    fn status(&self) -> Result<Vec<Item>>;
    fn check(&self) -> Result<()>;
    fn version(&self) -> Result<String>;
}

#[derive(QueryableByName)]
pub struct Table {
    #[sql_type = "Text"]
    pub name: String,
}

#[derive(QueryableByName)]
pub struct Version {
    #[sql_type = "Text"]
    pub value: String,
}

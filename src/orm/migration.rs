use diesel::sql_types::Text;

use super::super::errors::Result;

pub trait Dao {
    fn version(&self) -> Result<String>;
}

#[derive(QueryableByName)]
pub struct Version {
    #[sql_type = "Text"]
    pub value: String,
}

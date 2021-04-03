pub mod models;
pub mod schema;

use super::super::orm::migration::New as Migration;

#[derive(Clone)]
pub struct Plugin {}

impl super::Plugin for Plugin {
    fn migrations<'a>() -> Vec<Migration<'a>> {
        vec![Migration {
            version: "20210114174617",
            name: "init-iapt",
            up: include_str!("init-up.sql"),
            down: include_str!("init-down.sql"),
        }]
    }
}

pub mod models;
pub mod schema;

use super::super::super::orm::migration::New as Migration;

#[derive(Clone)]
pub struct Plugin {}

impl super::super::Plugin for Plugin {
    fn migrations<'a>() -> Vec<Migration<'a>> {
        vec![Migration {
            version: "20210113142934",
            name: "create-crawler",
            up: include_str!("up.sql"),
            down: include_str!("down.sql"),
        }]
    }
}

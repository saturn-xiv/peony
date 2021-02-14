pub mod models;
pub mod schema;

use super::super::orm::migration::New as Migration;

#[derive(Clone)]
pub struct Plugin {}

impl super::Plugin for Plugin {
    fn migrations<'a>() -> Vec<Migration<'a>> {
        let mut items = Vec::new();
        items.push(Migration {
            version: "20201214175333",
            name: "init-forum",
            up: include_str!("init-up.sql"),
            down: include_str!("init-down.sql"),
        });
        items
    }
}

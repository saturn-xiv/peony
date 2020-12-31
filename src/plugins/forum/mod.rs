pub mod models;
pub mod rpc;
pub mod schema;

use super::super::orm::migration::New as Migration;

#[derive(Clone)]
pub struct Plugin {}

impl super::Plugin for Plugin {
    fn migrations<'a>() -> Vec<Migration<'a>> {
        let mut items = Vec::new();
        items.push(Migration {
            version: "20201214175333",
            name: "create-forum",
            up: include_str!("create-up.sql"),
            down: include_str!("create-down.sql"),
        });
        items
    }
}

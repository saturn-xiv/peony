pub mod models;
pub mod schema;

use std::sync::Arc;

use super::super::super::{env::Context, orm::migration::New as Migration};

#[derive(Clone)]
pub struct Plugin {
    pub ctx: Arc<Context>,
}

impl super::super::Plugin for Plugin {
    fn migrations<'a>() -> Vec<Migration<'a>> {
        let mut items = Vec::new();
        items.push(Migration {
            version: "20210113142934",
            name: "create-crawler",
            up: include_str!("up.sql"),
            down: include_str!("down.sql"),
        });
        items
    }
}

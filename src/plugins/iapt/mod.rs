pub mod models;
pub mod rpc;
pub mod schema;

use std::sync::Arc;

use super::super::{env::Context, orm::migration::New as Migration};

#[derive(Clone)]
pub struct Plugin {
    pub ctx: Arc<Context>,
}

impl super::Plugin for Plugin {
    fn migrations<'a>() -> Vec<Migration<'a>> {
        let mut items = Vec::new();
        items.push(Migration {
            version: "20210114174617",
            name: "init-iapt",
            up: include_str!("init-up.sql"),
            down: include_str!("init-down.sql"),
        });
        items
    }
}

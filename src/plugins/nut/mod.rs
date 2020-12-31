pub mod models;
pub mod rpc;
pub mod schema;

use super::super::{i18n, orm::migration::New as Migration, settings};

#[derive(Clone)]
pub struct Plugin {}

impl super::Plugin for Plugin {
    fn migrations<'a>() -> Vec<Migration<'a>> {
        let mut items = Vec::new();
        items.push(Migration {
            version: "20201212175056",
            name: "create-settings",
            up: settings::UP,
            down: settings::DOWN,
        });
        items.push(Migration {
            version: "20201212175102",
            name: "create-locales",
            up: i18n::UP,
            down: i18n::DOWN,
        });
        items.push(Migration {
            version: "20201214174607",
            name: "create-auth",
            up: include_str!("create-auth-up.sql"),
            down: include_str!("create-auth-down.sql"),
        });
        items.push(Migration {
            version: "20201214174759",
            name: "create-site",
            up: include_str!("create-site-up.sql"),
            down: include_str!("create-site-down.sql"),
        });
        items
    }
}

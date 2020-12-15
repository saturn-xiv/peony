use super::super::{i18n, orm::migration::New as Migration, settings};

pub struct Plugin {}

impl super::Plugin for Plugin {
    fn migrations(&self) -> Vec<Migration> {
        let mut items = Vec::new();
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
        items.push(Migration {
            version: "20201214175056",
            name: "create-settings",
            up: settings::UP,
            down: settings::DOWN,
        });
        items.push(Migration {
            version: "20201214175102",
            name: "create-locales",
            up: i18n::UP,
            down: i18n::DOWN,
        });
        items
    }
}

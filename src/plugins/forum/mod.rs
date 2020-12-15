use super::super::orm::migration::New as Migration;

pub struct Plugin {}

impl super::Plugin for Plugin {
    fn migrations(&self) -> Vec<Migration> {
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

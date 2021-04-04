pub mod controllers;
pub mod models;
pub mod request;
pub mod schema;

use super::super::{i18n, orm::migration::New as Migration, settings};

#[derive(Clone)]
pub struct Plugin {}

impl super::Plugin for Plugin {
    fn migrations<'a>() -> Vec<Migration<'a>> {
        vec![]
    }
}

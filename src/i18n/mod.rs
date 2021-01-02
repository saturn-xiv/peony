pub mod locale;
pub mod schema;

pub const UP: &str = include_str!("up.sql");
pub const DOWN: &str = include_str!("down.sql");

use actix_web::http::StatusCode;
use handlebars::Handlebars;
use serde::ser::Serialize;

use super::{errors::Error, orm::postgresql::Connection};

use self::locale::Dao;

pub trait I18n {
    fn exist(&self, lang: &str) -> bool;
    fn tr<S: Serialize>(&self, lang: &str, code: &str, args: &Option<S>) -> Option<String>;
    fn e<C: Into<String>, S: Serialize>(&self, lang: &str, code: C, args: &Option<S>) -> Error;
    fn t<C: Into<String>, S: Serialize>(&self, lang: &str, code: C, args: &Option<S>) -> String;
}

impl I18n for Connection {
    fn exist(&self, lang: &str) -> bool {
        if let Ok(items) = Dao::languages(self) {
            return items.contains(&lang.to_string());
        }
        false
    }

    fn tr<S: Serialize>(&self, lang: &str, code: &str, args: &Option<S>) -> Option<String> {
        let tpl = Handlebars::new();
        if let Ok(it) = Dao::by_lang_and_code(self, lang, code) {
            if let Ok(msg) = tpl.render_template(&it.message, args) {
                return Some(msg);
            }
        }
        None
    }

    fn e<C: Into<String>, S: Serialize>(&self, lang: &str, code: C, args: &Option<S>) -> Error {
        let msg = self.t(lang, code, args);
        Error::Http(StatusCode::INTERNAL_SERVER_ERROR, Some(msg))
    }

    fn t<C: Into<String>, S: Serialize>(&self, lang: &str, code: C, args: &Option<S>) -> String {
        let code = code.into();
        match self.tr(lang, &code, args) {
            Some(msg) => msg,
            None => format!("{}.{}", lang, code),
        }
    }
}

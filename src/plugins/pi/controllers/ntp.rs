use std::fs::File;
use std::io::prelude::*;
use std::ops::Deref;
use std::path::{Component, Path};

use actix_web::{get, http::StatusCode, post, web, HttpResponse, Responder};
use chrono::{DateTime, Local};
use chrono_tz::Tz;
use validator::Validate;

use super::super::super::super::{
    cache::{redis::Pool as DbPool, Kv},
    errors::{Error, Result},
    sys::ntp::Response as NtpResponse,
};
use super::CurrentUser;

#[derive(Validate, Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    #[validate(length(min = 1))]
    pub timezone: String,
    #[validate(length(min = 1))]
    pub server: String,
}

impl Default for Form {
    fn default() -> Self {
        Self {
            timezone: Tz::UTC.name().to_string(),
            server: "0.us.pool.ntp.org".to_string(),
        }
    }
}

impl Form {
    pub const KEY: &'static str = "ntp";

    pub fn timesyncd(&self) -> String {
        format!(
            r#"
[Time]
NTP={server}
FallbackNTP=0.pool.ntp.org 1.pool.ntp.org 2.pool.ntp.org 3.pool.ntp.org
        "#,
            server = self.server
        )
    }
    pub fn crontab(&self) -> String {
        format!(
            r#"
#!/bin/sh
ntpdate {server}
        "#,
            server = self.server
        )
    }

    pub fn test(&self) -> Result<(DateTime<Local>, Tz)> {
        let tz = self.timezone.parse::<Tz>().map_err(|e| {
            Error::Http(StatusCode::BAD_REQUEST, Some(format!("bad timezone {}", e)))
        })?;
        let now = NtpResponse::fetch(&self.server, None)?;
        Ok((now.into(), tz))
    }

    pub fn save(&self) -> Result<()> {
        debug!("save ntp server {:?}", self);
        let mut fd = File::create(
            Path::new(&Component::RootDir)
                .join("etc")
                .join("systemd")
                .join("timesyncd.conf"),
        )?;
        write!(&mut fd, "{}", self.timesyncd())?;
        Ok(())
    }
}

#[get("/")]
pub async fn get(_user: CurrentUser, db: web::Data<DbPool>) -> Result<impl Responder> {
    let db = db.deref();
    let db = db.deref();
    let it: Form = Kv::get(db, &Form::KEY.to_string()).unwrap_or_default();
    Ok(HttpResponse::Ok().json(it))
}

#[post("/")]
pub async fn set(
    _user: CurrentUser,
    form: web::Json<Form>,
    db: web::Data<DbPool>,
) -> Result<impl Responder> {
    let db = db.deref();
    let db = db.deref();
    let form = form.deref();
    form.validate()?;
    form.test()?;
    form.save()?;
    Kv::set(db, &Form::KEY.to_string(), form)?;
    Ok(HttpResponse::Ok().json(()))
}

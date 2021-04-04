pub mod models;
pub mod schema;

use std::fs::File;
use std::io::prelude::*;
use std::path::{Component, Path};

use super::super::super::{errors::Result, orm::postgresql::Connection as Db};

// https://crontab.guru/
// https://help.ubuntu.com/community/CronHowto
#[derive(Clone)]
pub struct Plugin {}

impl super::super::Plugin for Plugin {}

impl Plugin {
    pub fn save(&self, name: &str, db: &Db) -> Result<()> {
        use self::models::task::Dao as TaskDao;

        let file = Path::new(&Component::RootDir)
            .join("etc")
            .join("cron.d")
            .join(name);
        debug!("write crontab into {}", file.display());
        let mut file = File::open(&file)?;
        for it in TaskDao::all(db)? {
            writeln!(file, "{}", it)?;
        }
        Ok(())
    }
}

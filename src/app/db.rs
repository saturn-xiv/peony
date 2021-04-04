use super::super::{errors::Result, orm::postgresql::Connection};

pub fn migrate(db: &Connection) -> Result<()> {
    Ok(())
}

pub fn rollback(db: &Connection) -> Result<()> {
    Ok(())
}

pub fn reset(db: &Connection) -> Result<()> {
    Ok(())
}

pub fn status(db: &Connection) -> Result<()> {
    // if matches.subcommand_matches("status").is_some() {
    //     println!("{:<14} {:<32} RUN AT", "VERSION", "NAME");
    //     for it in db.status()? {
    //         println!("{}", it);
    //     }
    //     return Ok(());
    // }
    Ok(())
}

use super::super::{errors::Result, orm::postgresql::Connection};

// embed_migrations!("db/migrations/postgresql");

pub fn migrate(db: &Connection) -> Result<()> {
    // embedded_migrations::run(db)?;
    Ok(())
}

pub fn rollback(_db: &Connection) -> Result<()> {
    // TODO
    Ok(())
}

pub fn reset(_db: &Connection) -> Result<()> {
    // TODO
    Ok(())
}

pub fn status(_db: &Connection) -> Result<()> {
    // TODO
    // if matches.subcommand_matches("status").is_some() {
    //     println!("{:<14} {:<32} RUN AT", "VERSION", "NAME");
    //     for it in db.status()? {
    //         println!("{}", it);
    //     }
    //     return Ok(());
    // }
    Ok(())
}

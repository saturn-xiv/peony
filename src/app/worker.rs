use super::super::{env::Config, errors::Result};

pub fn launch(_cfg: &Config, queue: &str) -> Result<()> {
    info!("start worker for {}", queue);
    // TODO
    Ok(())
}

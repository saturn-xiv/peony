use chrono::{NaiveDateTime, Utc};

use super::super::super::{errors::Result, protos::hal::GpioPressedMode};

pub struct Button {
    pub id: u32,
}

pub fn touch(_id: u8, _begin: &NaiveDateTime) -> Result<Option<GpioPressedMode>> {
    // TODO
    let _now = Utc::now().naive_local();
    Ok(None)
}

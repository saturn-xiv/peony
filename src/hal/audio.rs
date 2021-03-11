use sdl2::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::time::Duration;

use super::super::errors::Result;

pub struct Audio {}

impl Audio {
    pub fn play(&mut self) -> Result<()> {
        // TODO
        Ok(())
    }
    pub fn stop(&mut self) -> Result<()> {
        // TODO
        Ok(())
    }
}

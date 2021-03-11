use sdl2::audio::{AudioCVT, AudioCallback, AudioSpecDesired, AudioSpecWAV};
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::time::Duration;

use super::super::errors::Result;

struct Sound {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for Sound {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            let pre_scale = *self.data.get(self.pos).unwrap_or(&128);
            let scaled_signed_float = (pre_scale as f32 - 128.0) * self.volume;
            let scaled = (scaled_signed_float + 128.0) as u8;
            *dst = scaled;
            self.pos += 1;
        }
    }
}

pub struct Audio {}

impl Audio {
    pub fn new()->Result<Self>{

    }
    pub fn play(&mut self) -> Result<()> {
        // TODO
        Ok(())
    }
    pub fn stop(&mut self) -> Result<()> {
        // TODO
        Ok(())
    }
}

use std::path::Path;
use std::thread;
use std::time::Duration;

use sdl2::mixer::{self, InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

use super::super::errors::Result;

pub fn init() -> Result<()> {
    debug!("sdl version {}", sdl2::version::version());
    debug!("sdl mixer version {}", mixer::get_linked_version());

    let sdl = sdl2::init()?;
    sdl.audio()?;
    mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1_024)?;
    mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;
    mixer::allocate_channels(4);
    {
        let n = mixer::get_chunk_decoders_number();
        debug!("available chunk(sample) decoders: {}", n);
        for i in 0..n {
            debug!("decoder {} => {}", i, mixer::get_chunk_decoder(i));
        }
    }
    {
        let n = mixer::get_music_decoders_number();
        debug!("available music decoders: {}", n);
        for i in 0..n {
            debug!("decoder {} => {}", i, mixer::get_music_decoder(i));
        }
    }
    debug!("spec => {:?}", mixer::query_spec());
    fn hook_finished() {
        debug!("play music ends!");
    }
    mixer::Music::hook_finished(hook_finished);
    Ok(())
}

pub fn play<T: AsRef<Path>>(file: T, volume: u8, loops: usize) -> Result<()> {
    debug!("sdl version {}", sdl2::version::version());
    debug!("sdl mixer version {}", mixer::get_linked_version());

    let sdl = sdl2::init()?;
    let _ = sdl.audio()?;
    mixer::open_audio(44_100, AUDIO_S16LSB, DEFAULT_CHANNELS, 1_024)?;
    let _ = mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;
    mixer::allocate_channels(4);
    {
        let n = mixer::get_chunk_decoders_number();
        debug!("available chunk(sample) decoders: {}", n);
        for i in 0..n {
            debug!("decoder {} => {}", i, mixer::get_chunk_decoder(i));
        }
    }
    {
        let n = mixer::get_music_decoders_number();
        debug!("available music decoders: {}", n);
        for i in 0..n {
            debug!("decoder {} => {}", i, mixer::get_music_decoder(i));
        }
    }
    debug!("spec => {:?}", mixer::query_spec());
    fn hook_finished() {
        debug!("play music ends",);
    }
    mixer::Music::hook_finished(hook_finished);

    mixer::Music::set_volume(volume as i32);
    let file = file.as_ref();
    debug!("mucic file => {}", file.display());
    let music = mixer::Music::from_file(file)?;
    debug!("music type => {:?}", music.get_type());
    debug!("music volume => {}", mixer::Music::get_volume());
    music.play(loops as i32)?;

    loop {
        if !mixer::Music::is_playing() {
            thread::sleep(Duration::from_micros(10));
            break;
        }
    }
    Ok(())
}
pub fn pause() {
    mixer::Music::pause();
}
pub fn is_playing() -> bool {
    mixer::Music::is_playing()
}

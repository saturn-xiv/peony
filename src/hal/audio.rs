use std::path::Path;

use actix_web::http::StatusCode;

use sdl2::{
    audio::{
        AudioCVT, AudioCallback, AudioDevice, AudioSpec, AudioSpecDesired, AudioSpecWAV,
        AudioStatus,
    },
    mixer::{get_linked_version, InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS},
};

use super::super::errors::{Error, Result};

pub struct Music {}

impl Music {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
    pub fn play(&self) -> Result<()> {
        info!("mixer version: {}", get_linked_version());

        let sdl = sdl2::init()?;
        sdl.audio()?;
        // let mut timer = sdl.timer()?;

        // let frequency = 44_100;
        // let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
        // let channels = DEFAULT_CHANNELS; // Stereo
        // let chunk_size = 1_024;
        // sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
        // let _mixer_context =
        //     sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;

        // // Number of mixing channels available for sound effect `Chunk`s to play
        // // simultaneously.
        // sdl2::mixer::allocate_channels(4);

        // {
        //     let n = sdl2::mixer::get_chunk_decoders_number();
        //     println!("available chunk(sample) decoders: {}", n);
        //     for i in 0..n {
        //         println!("  decoder {} => {}", i, sdl2::mixer::get_chunk_decoder(i));
        //     }
        // }

        // {
        //     let n = sdl2::mixer::get_music_decoders_number();
        //     println!("available music decoders: {}", n);
        //     for i in 0..n {
        //         println!("  decoder {} => {}", i, sdl2::mixer::get_music_decoder(i));
        //     }
        // }

        // println!("query spec => {:?}", sdl2::mixer::query_spec());

        // let music = sdl2::mixer::Music::from_file(music_file)?;

        // fn hook_finished() {
        //     println!("play ends! from rust cb");
        // }

        // sdl2::mixer::Music::hook_finished(hook_finished);

        // println!("music => {:?}", music);
        // println!("music type => {:?}", music.get_type());
        // println!("music volume => {:?}", sdl2::mixer::Music::get_volume());
        // println!("play => {:?}", music.play(1));

        // {
        //     let sound_chunk = match sound_file {
        //         Some(sound_file_path) => sdl2::mixer::Chunk::from_file(sound_file_path)
        //             .map_err(|e| format!("Cannot load sound file: {:?}", e))?,
        //         None => {
        //             // One second of 500Hz sine wave using equation A * sin(2 * PI * f * t)
        //             // (played at half the volume to save people's ears).
        //             let buffer = (0..frequency)
        //                 .map(|i| {
        //                     (0.1 * i16::max_value() as f32
        //                         * (2.0 * 3.14 * 500.0 * (i as f32 / frequency as f32)).sin())
        //                         as i16
        //                 })
        //                 .collect();
        //             sdl2::mixer::Chunk::from_raw_buffer(buffer)
        //                 .map_err(|e| format!("Cannot get chunk from buffer: {:?}", e))?
        //         }
        //     };

        //     println!("chunk volume => {:?}", sound_chunk.get_volume());
        //     println!("playing sound twice");
        //     sdl2::mixer::Channel::all().play(&sound_chunk, 1)?;

        //     // This delay is needed because when the `Chunk` goes out of scope,
        //     // the sound effect stops playing. Delay long enough to hear the
        //     // sound.
        //     timer.delay(5_000);
        //     println!("played sound");
        // }

        // timer.delay(10_000);

        // println!("fading out ... {:?}", sdl2::mixer::Music::fade_out(4_000));

        // timer.delay(5_000);

        // println!(
        //     "fading in from pos ... {:?}",
        //     music.fade_in_from_pos(1, 10_000, 100.0)
        // );

        // timer.delay(5_000);
        // sdl2::mixer::Music::halt();
        // timer.delay(1_000);

        // println!("quitting sdl");

        Ok(())
    }
}

pub struct Sound {
    pub data: Vec<u8>,
    pub volume: f32,
    pub pos: usize,
}

impl Sound {
    pub fn wav<T: AsRef<Path>>(file: T, volume: f32, spec: &AudioSpec) -> Result<Self> {
        let file = file.as_ref();
        debug!("load file {}", file.display());
        let wav = AudioSpecWAV::load_wav(file)
            .map_err(|x| Error::Http(StatusCode::INTERNAL_SERVER_ERROR, Some(x)))?;

        let cvt = AudioCVT::new(
            wav.format,
            wav.channels,
            wav.freq,
            spec.format,
            spec.channels,
            spec.freq,
        )
        .map_err(|x| Error::Http(StatusCode::INTERNAL_SERVER_ERROR, Some(x)))?;

        let data = cvt.convert(wav.buffer().to_vec());
        Ok(Self {
            data,
            volume,
            pos: 0,
        })
    }
}
impl Default for Sound {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            volume: 0.25,
            pos: 0,
        }
    }
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

pub struct Audio {
    pub device: AudioDevice<Sound>,
}

impl Audio {
    pub fn new() -> Result<Self> {
        let sdl_context =
            sdl2::init().map_err(|x| Error::Http(StatusCode::INTERNAL_SERVER_ERROR, Some(x)))?;
        let audio_subsystem = sdl_context
            .audio()
            .map_err(|x| Error::Http(StatusCode::INTERNAL_SERVER_ERROR, Some(x)))?;

        let device = audio_subsystem
            .open_playback(
                None,
                &AudioSpecDesired {
                    freq: Some(44_100),
                    channels: Some(1),
                    samples: None,
                },
                |_| Sound::default(),
            )
            .map_err(|x| Error::Http(StatusCode::INTERNAL_SERVER_ERROR, Some(x)))?;
        device.resume();
        Ok(Self { device })
    }

    pub fn play<T: AsRef<Path>>(&mut self, file: T, volume: f32) -> Result<()> {
        let data = Sound::wav(file, volume, self.device.spec())?;
        let mut lock = self.device.lock();
        *lock = data;
        Ok(())
    }

    pub fn is_playing(&self) -> bool {
        self.device.status() == AudioStatus::Playing
    }

    pub fn stop(&self) -> Result<()> {
        self.device.pause();
        Ok(())
    }
}

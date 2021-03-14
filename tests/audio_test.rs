// extern crate peony;
extern crate sdl2;

use std::path::Path;
use std::thread;
use std::time::Duration;

use peony::hal::audio;
use sdl2::mixer::{InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS};

#[test]
fn test_wav() {
    env_logger::init();
    let root = Path::new("tmp").join("wav");
    // demo(&root.join("1.wav")).unwrap();

    // audio::init().unwrap();

    println!("linked version: {}", sdl2::mixer::get_linked_version());

    let sdl = sdl2::init().unwrap();
    let _audio = sdl.audio().unwrap();

    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();
    let _mixer_context =
        sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG).unwrap();
    for it in vec!["0.wav", "1.wav", "2.wav", "3.wav"] {
        let music = sdl2::mixer::Music::from_file(&root.join(it)).unwrap();

        println!("music => {}", it);
        println!("music type => {:?}", music.get_type());
        println!("music volume => {:?}", sdl2::mixer::Music::get_volume());
        music.play(1).unwrap();
        loop {
            if !sdl2::mixer::Music::is_playing() {
                thread::sleep(Duration::from_secs(1));
                break;
            }
        }
        // demo(&root.join(it)).unwrap();
        // audio::play(root.join(it), 64, 1).unwrap();
        // loop {
        //     if !audio::is_playing() {
        //         thread::sleep(Duration::from_secs(10));
        //         break;
        //     }
        // }
    }
}

fn demo(music_file: &Path) -> Result<(), String> {
    println!("linked version: {}", sdl2::mixer::get_linked_version());

    let sdl = sdl2::init()?;
    let _audio = sdl.audio()?;

    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;
    let _mixer_context =
        sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;

    // Number of mixing channels available for sound effect `Chunk`s to play
    // simultaneously.
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

    println!("query spec => {:?}", sdl2::mixer::query_spec());

    let music = sdl2::mixer::Music::from_file(music_file)?;

    fn hook_finished() {
        println!("play ends! from rust cb");
    }

    sdl2::mixer::Music::hook_finished(hook_finished);
    sdl2::mixer::Music::set_volume(64);

    println!("music => {:?}", music);
    println!("music type => {:?}", music.get_type());
    println!("music volume => {:?}", sdl2::mixer::Music::get_volume());
    println!("play => {:?}", music.play(1));

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

    loop {
        if !sdl2::mixer::Music::is_playing() {
            thread::sleep(Duration::from_secs(1));
            break;
        }
    }

    println!("quitting sdl");

    Ok(())
}

// #[test]
// fn test_wav() {
//     let root = Path::new("tmp").join("wav");
//     let mut audio = Audio::new().unwrap();
//     for it in vec!["1.wav", "2.wav", "3.wav"] {
//         println!("play {}", it);
//         audio.play(root.join(it), 0.75).unwrap();
//         loop {
//             println!("{} {:?}", it, audio.device.status());
//             thread::sleep(Duration::from_secs(10));
//             if !audio.is_playing() {
//                 break;
//             }
//         }
//     }
// }

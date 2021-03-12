extern crate peony;
extern crate sdl2;

use std::path::Path;
use std::thread;
use std::time::Duration;

use peony::hal::audio::Audio;

#[test]
fn test_wav() {
    let root = Path::new("tmp").join("wav");
    let mut audio = Audio::new().unwrap();
    for it in vec!["1.wav", "2.wav", "3.wav"] {
        println!("play {}", it);
        audio.play(root.join(it), 0.75).unwrap();
        loop {
            println!("{} {:?}", it, audio.device.status());
            thread::sleep(Duration::from_secs(10));
            if !audio.is_playing() {
                break;
            }
        }
    }
}

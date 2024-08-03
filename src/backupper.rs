use crate::{audio::{play_audio_file, play_audio_sin}, config::Config};

pub struct Backupper {
    config: Config,
}

impl Backupper {
    pub fn new() -> Self {
        Backupper {
            config: Config::new(),
        }
    }

    pub fn init(&self) {
        println!("Backup initialized, waiting for confirm...");
        play_audio_file("start_sound.wav");
    }

    pub fn confirm(&self) {
        println!("Backup confirmed, starting...");
        play_audio_sin(1000.0, 0.1);
    }

    pub fn cancel(&self) {
        println!("Backup canceled");
        play_audio_sin(300.0, 0.5);
    }
}
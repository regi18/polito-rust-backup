use crate::audio::{play_audio_sin, play_audio_file};

pub struct Backupper {

}

impl Backupper {
    pub fn new() -> Self {
        Backupper {}
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
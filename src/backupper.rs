use crate::utils::play_audio;

pub struct Backupper {

}

impl Backupper {
    pub fn new() -> Self {
        Backupper {}
    }

    pub fn init(&self) {
        println!("Backup initialized, waiting for confirm...");
        play_audio("start_sound.wav");
    }

    pub fn confirm(&self) {
        
    }

    pub fn cancel(&self) {

    }
}
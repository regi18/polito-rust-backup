use std::{fs, io, path::Path};

use crate::{audio::{play_audio_file, play_audio_sin}, config::Config};

/**
  Recursively copies src folder into dst
 */
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } 
        else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}




#[derive(PartialEq)]
enum BackupperStatus {
    Ready,
    WaitingConfirm,
    Running,
}

pub struct Backupper {
    config: Config,
    status: BackupperStatus,
}

impl Backupper {
    pub fn new() -> Self {
        Backupper {
            config: Config::new(),
            status: BackupperStatus::Ready,
        }
    }

    pub fn init(&mut self) {
        println!("[*] Backup initialized, waiting for confirm...");
        play_audio_file("start_sound.wav");

        self.status = BackupperStatus::WaitingConfirm;
    }

    pub fn confirm(&mut self) {
        if self.status != BackupperStatus::WaitingConfirm {
            return;
        }

        println!("[*] Backup confirmed, starting...");
        play_audio_sin(1000.0, 0.1);

        self.status = BackupperStatus::Running;

        copy_dir_all(&self.config.backup_source, &self.config.backup_dest).unwrap();

        println!("[+] Backup finished!");
        self.status = BackupperStatus::Ready;
    }

    pub fn cancel(&mut self) {
        println!("[!] Backup canceled");
        play_audio_sin(300.0, 0.5);

        self.status = BackupperStatus::Ready;
    }
}
use crate::{
    audio::{play_audio_file, play_audio_sin}, config::Config, cpu_logger::ProcessTime
};
use std::{ffi::OsStr, fs, io, path::Path};

/**
 Recursively copies src folder into dst
*/
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>, file_types: &[String]) -> io::Result<u64> {
    fs::create_dir_all(&dst)?;

    let mut total_size = 0;
    
    // Check if we need to copy all files
    let copy_all_files = file_types.contains(&"*".to_string());

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            let subdir_dst = dst.as_ref().join(entry.file_name());
            // Recursively copy subdirectories
            total_size += copy_dir_all(entry.path(), &subdir_dst, file_types)?;
        }
        else {
            let path = entry.path();
            let extension = path.extension().and_then(OsStr::to_str).unwrap_or("");
            if copy_all_files || file_types.is_empty() || file_types.contains(&extension.to_string()) {
                fs::copy(&path, dst.as_ref().join(entry.file_name()))?;
                let metadata = fs::metadata(&path)?;
                total_size += metadata.len();
            }
        }
    }

    Ok(total_size)
}

#[derive(PartialEq, Clone, Debug)]
pub enum BackupperStatus {
    Ready,
    WaitingConfirm,
    Running,
}

#[derive(Clone)]
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
    
    pub fn get_status(&self) -> BackupperStatus {
        self.status.clone()
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

        // Start measuring CPU time
        let start_cpu_time = ProcessTime::now();

        // Back-up operation
        let (total_size, result) = {
            let src = &self.config.backup_source;
            let dst = &self.config.backup_dest;
            let file_types = &self.config.file_types;
            let result = copy_dir_all(src, dst, file_types);
            let total_size = result.as_ref().map_or(0, |size| *size);

            (total_size, result)
        };

        // End measuring CPU time
        let elapsed_cpu_time = start_cpu_time.elapsed();

        match result {
            Ok(_) => println!("[+] Backup finished"),
            Err(e) => {
                eprintln!("[!] Backup failed: {}", e);
                self.status = BackupperStatus::Ready;
                return;
            }
        }

        //save CPU info in log file
        let backup_dest_path = Path::new(&self.config.backup_dest);
        let log_file_path = backup_dest_path.join("backup_log.txt");
        let log_message = format!(
            "Backup completed successfully.\nTotal size: {} bytes\nCPU time used: {:?}\n",
            total_size, elapsed_cpu_time
        );

        if let Err(e) = fs::write(log_file_path, log_message) {
            eprintln!("[!] Failed to write log file: {}", e);
        }

        self.status = BackupperStatus::Ready;
    }

    pub fn cancel(&mut self) {
        println!("[!] Backup canceled");
        play_audio_sin(300.0, 0.5);

        self.status = BackupperStatus::Ready;
    }
}




#[cfg(test)]
mod backupper_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_copy_dir_all() {
        let src = PathBuf::from("test_data/backup_src");
        let dst = PathBuf::from("test_data/backup_dst");
        let file_types = vec!["txt".to_string()];

        let result = copy_dir_all(src, dst, &file_types);
        assert!(result.is_ok());
        assert!(result.unwrap() > 0);
    }

    #[test]
    fn test_backup_init() {
        let mut backupper = Backupper::new();
        backupper.init();
        assert_eq!(backupper.status, BackupperStatus::WaitingConfirm);
    }

    #[test]
    fn test_backup_confirm() {
        let mut backupper = Backupper::new();
        backupper.init();
        backupper.confirm();
        assert_eq!(backupper.status, BackupperStatus::Ready);
    }

    #[test]
    fn test_backup_cancel() {
        let mut backupper = Backupper::new();
        backupper.init();
        backupper.cancel();
        assert_eq!(backupper.status, BackupperStatus::Ready);
    }
}
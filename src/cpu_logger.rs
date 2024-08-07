use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io;
use std::thread;
use std::time::Duration;
use sysinfo::{Pid, ProcessExt, System, SystemExt};
use libc::{clock_gettime, timespec, CLOCK_PROCESS_CPUTIME_ID};

pub struct Logger {
    log_file: File,
    cpu_logging_interval_secs: u64,
}

impl Logger {
    pub fn new(cpu_logging_interval_secs: u64) -> Self {
        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("cpu_usage.log")
            .expect("Failed to open log file");

        Logger::clear_log_file(&mut log_file);

        Logger {
            log_file,
            cpu_logging_interval_secs,
        }
    }

    pub fn start(&mut self) {
        let cpu_logging_interval_secs = self.cpu_logging_interval_secs.clone();
        let mut file = self.log_file.try_clone().unwrap();

        thread::spawn(move || {
            let mut sys = System::new_all();
            let pid = Pid::from(std::process::id() as usize);

            loop {
                sys.refresh_process(pid);

                if let Some(process) = sys.process(pid) {
                    let cpu_usage = process.cpu_usage();
                    let log_entry = format!("CPU Usage: {}%\n", cpu_usage);

                    // Append CPU usage to log file
                    file.write_all(log_entry.as_bytes()).expect("Failed to write to log file");
                }

                thread::sleep(Duration::from_secs(cpu_logging_interval_secs));
            }
        });
    }

    fn clear_log_file(log_file: &mut File) {
        log_file.set_len(0).unwrap();
        log_file.write_all(b"Starting CPU logging...\n").expect("Failed to write to log file");
    }
}


/// CPU Time Used by The Whole Process
///
/// This is an opaque type similar to `std::time::Instant`.
/// Use `elapsed()` or `duration_since()` to get meaningful time deltas.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct ProcessTime(Duration);

impl ProcessTime {
    /// Get current CPU time used by a process
    pub fn try_now() -> io::Result<Self> {
        let mut time = timespec { tv_sec: 0, tv_nsec: 0 };
        if unsafe { clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &mut time) } == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(ProcessTime(Duration::new(time.tv_sec as u64, time.tv_nsec as u32)))
    }

    /// Get current CPU time used by a process
    pub fn now() -> Self {
        Self::try_now().expect("CLOCK_PROCESS_CPUTIME_ID unsupported")
    }

    /// Returns the amount of CPU time used from the previous timestamp to now.
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }

    /// Returns the amount of CPU time used from the previous timestamp.
    pub fn duration_since(&self, timestamp: Self) -> Duration {
        self.0 - timestamp.0
    }
}



#[cfg(test)]
mod cpu_logger_tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_logger_creation() {
        let _ = Logger::new(120);
        assert!(fs::metadata("cpu_usage.log").is_ok());
    }

    #[test]
    fn test_logger_start() {
        let mut logger = Logger::new(1);
        logger.start();
        // Wait a bit to let the logger write
        std::thread::sleep(std::time::Duration::from_secs(2));
        assert!(fs::read_to_string("cpu_usage.log").unwrap().contains("CPU Usage:"));
    }
}
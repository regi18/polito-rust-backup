use std::fs::OpenOptions;
use std::io::Write;
use std::io;
use std::thread;
use std::time::Duration;
use sysinfo::{Pid, ProcessExt, System, SystemExt};
use libc::{clock_gettime, timespec, CLOCK_PROCESS_CPUTIME_ID};

pub fn start_cpu_logging() {
    thread::spawn(|| {
        let mut sys = System::new_all();
        let pid = Pid::from(std::process::id() as usize);
        loop {
            sys.refresh_process(pid);
            if let Some(process) = sys.process(pid) {
                let cpu_usage = process.cpu_usage();
                let log_entry = format!("CPU Usage: {}%\n", cpu_usage);

                // Append CPU usage to log file
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("cpu_usage.log")
                    .expect("Failed to open log file");
                file.write_all(log_entry.as_bytes())
                    .expect("Failed to write to log file");
            }
            thread::sleep(Duration::from_secs(30)); // Sleep for 2 minutes
        }
    });
}


pub fn clear_cpu_log_file() {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("cpu_usage.log")
        .expect("Failed to open log file");
    file.write_all(b"Starting CPU logging...\n")
        .expect("Failed to write to log file");
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


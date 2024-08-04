use backupper::Backupper;
use handle_figure_recognition::recognize_figures;
use cpu_logger::{start_cpu_logging, clear_cpu_log_file};

mod figures_templates;
mod guessture;
mod handle_figure_recognition;
mod backupper;
mod audio;
mod config;
mod cpu_logger;


fn main() {
    // Clear the CPU log file at the start of the program
    clear_cpu_log_file();

    // Start CPU logging in a separate thread
    start_cpu_logging();

    let mut backupper = Backupper::new();

    recognize_figures(|name| {
        match name.as_str() {
            "rectangle" => backupper.init(),
            "triangle" => backupper.confirm(),
            "delete" => backupper.cancel(),
            _ => {},
        }
    })
}
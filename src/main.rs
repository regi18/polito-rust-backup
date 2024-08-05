use backupper::Backupper;
use handle_figure_recognition::recognize_figures;
use cpu_logger::Logger;

mod figures_templates;
mod guessture;
mod handle_figure_recognition;
mod backupper;
mod audio;
mod config;
mod cpu_logger;
mod confirmation_dialog;


fn main() {
    // Start CPU logging in a separate thread
    let mut logger = Logger::new(120);
    logger.start();

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
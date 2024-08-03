use backupper::Backupper;
use handle_figure_recognition::recognize_figures;

mod figures_templates;
mod guessture;
mod handle_figure_recognition;
mod backupper;
mod audio;


fn main() {
    let backupper = Backupper::new();

    recognize_figures(|name| {
        match name.as_str() {
            "rectangle" => backupper.init(),
            "triangle" => backupper.confirm(),
            "delete" => backupper.cancel(),
            _ => {},
        }
    })
}
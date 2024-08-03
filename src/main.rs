use handle_figure_recognition::recognize_figures;

mod figures_templates;
mod guessture;
mod handle_figure_recognition;
mod Backupper;


fn start_backup() {

}


fn main() {
    let backupper = Backupper::new()

    // N.B. The drawing order of the figures must be the same as shown here "https://depts.washington.edu/acelab/proj/dollar/index.html"
    recognize_figures(|name| {
        match name.as_str() {
            "rectangle" => Backupper::start(),
            "triangle" => Backupper::confirm(),
            "delete" => Backupper::cancel(),
            _ => {},
        }
        println!("pattern: {}",  name);
    })
}
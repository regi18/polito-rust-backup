use std::sync::{mpsc, Arc, Mutex};

use backupper::Backupper;
use confirmation_dialog::ConfirmDialog;
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
    Logger::new(120).start();

    let confirm_dialog = Arc::new(ConfirmDialog::new());
    let backupper = Arc::new(Mutex::new(Backupper::new()));
    let (tx, rx) = mpsc::channel();

    let b = backupper.clone();
    let c = confirm_dialog.clone();
    let h = recognize_figures(move |name| {
        let mut guard = b.lock().unwrap();

        match name.as_str() {
            "rectangle" => { guard.init(); tx.send(()).unwrap(); },
            "triangle" => { c.close(); guard.confirm(); }
            "delete" => { c.close(); guard.cancel(); }
            _ => {},
        }
    });

    let c1 = confirm_dialog.clone();
    loop {
        let b1 = backupper.clone();

        match rx.recv() {
            Ok(_) => {
                c1.open(move |result, is_running| {
                    (*(*is_running).lock().unwrap()) = false;
                    let mut guard = b1.lock().unwrap();
                    match result {
                        true => guard.confirm(),
                        false => guard.cancel(),
                    };
                });
            },
            Err(e) => {
                println!("ERROR: {:?}", e);
                break;
            }
        };
    }

    h.join().unwrap();
}
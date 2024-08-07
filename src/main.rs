use std::sync::{mpsc, Arc, Mutex};

use backupper::Backupper;
use confirmation_dialog::ConfirmDialog;
use handle_figure_recognition::recognize_figures;
use cpu_logger::Logger;
use command_line::CommandLine;

mod command_line;
mod figures_templates;
mod guessture;
mod handle_figure_recognition;
mod backupper;
mod audio;
mod config;
mod cpu_logger;
mod confirmation_dialog;



fn main() {
    CommandLine::handle();
    Logger::new(120).start();

    let backupper = Arc::new(Mutex::new(Backupper::new()));
    let (tx, rx) = mpsc::channel();

    let b = backupper.clone();
    let h = recognize_figures(move |name| {
        let mut guard = b.lock().unwrap();

        match name.as_str() {
            "rectangle" => { guard.init(); tx.send(()).unwrap(); },
            "triangle" => { guard.confirm(); }
            "delete" => { guard.cancel(); }
            _ => {},
        }
    });


    loop {
        match rx.recv() {
            Ok(_) => {
                let res = ConfirmDialog::open();
                let mut guard = backupper.lock().unwrap();
                match res {
                    true => guard.confirm(),
                    false => guard.cancel(),
                };
            },
            Err(e) => {
                println!("CHANNEL ERROR: {:?}", e);
                break;
            }
        };
    }

    h.join().unwrap();
}


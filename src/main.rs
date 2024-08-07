use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

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

    let confirm_dialog = Arc::new(Mutex::new(ConfirmDialog::new()));
    let backupper = Arc::new(Mutex::new(Backupper::new()));
    let (tx, rx) = mpsc::channel();

    let b = backupper.clone();
    let c = confirm_dialog.clone();
    let h = recognize_figures(move |name| {
        println!("[DEBUG] Figure recognized: {}", name);

        let mut guard = b.lock().unwrap();
        let mut cg = c.lock().unwrap();

        match name.as_str() {
            "rectangle" => { guard.init(); tx.send(()).unwrap(); },
            "triangle" => { cg.close(); guard.confirm(); }
            "delete" => { cg.close(); guard.cancel(); }
            _ => {},
        }
    });


    // TESTING
    // confirm_dialog.lock().unwrap().open();
    // TESTING
    let c1 = confirm_dialog.clone();
    loop {
        let b1 = backupper.clone();

        match rx.recv() {
            Ok(_) => {
                let mut cg = c1.lock().unwrap();
                cg.open();
                drop(cg);

                loop {
                    let mut cg = c1.lock().unwrap();
                    let result = cg.check_result();

                    match result {
                        Some(e) => {
                            let mut guard = b1.lock().unwrap();
                            if e { guard.confirm(); }
                            else { guard.cancel(); }
                            
                            cg.close();
                            break;
                        }
                        _ => {},
                    };

                    drop(cg);
                    break;
                    // thread::sleep(Duration::from_millis(1000));
                }
            },
            Err(e) => {
                println!("CHANNEL ERROR: {:?}", e);
                break;
            }
        };
    }

    h.join().unwrap();
}


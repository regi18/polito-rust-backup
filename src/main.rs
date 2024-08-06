use std::sync::{mpsc, Arc, Mutex};
use clap::{App, Arg};

use backupper::Backupper;
use confirmation_dialog::ConfirmDialog;
use handle_figure_recognition::recognize_figures;
use cpu_logger::Logger;
use help::print_help;

mod help;
mod figures_templates;
mod guessture;
mod handle_figure_recognition;
mod backupper;
mod audio;
mod config;
mod cpu_logger;
mod confirmation_dialog;



fn main() {

    let matches = App::new("backup_tool")
        .about("A tool for creating backups using a visual command.")
        .arg(Arg::with_name("help")
            .short("h")
            .long("help")
            .help("Prints help information"))
        .get_matches();

    // verify if the help flag is present
    if matches.is_present("help") {
        print_help();
        return;
    }

    // Start CPU logging in a separate thread
    Logger::new(120).start();

    let confirm_dialog = Arc::new(ConfirmDialog::new());
    let backupper = Arc::new(Mutex::new(Backupper::new()));
    let (tx, rx) = mpsc::channel();

    // TESTING
    //confirm_dialog.open(|_,_|{});
    // TESTING

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
                println!("CHANNEL ERROR: {:?}", e);
                break;
            }
        };
    }

    h.join().unwrap();
}


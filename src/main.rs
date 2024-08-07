use std::sync::{Arc, Mutex};

use backupper::Backupper;
// use confirmation_dialog::ConfirmDialog;
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

    // let confirm_dialog = Arc::new(ConfirmDialog::new());
    let backupper = Arc::new(Mutex::new(Backupper::new()));
    // let (tx, rx) = mpsc::channel();

    let b = backupper.clone();
    // let c = confirm_dialog.clone();
    let h = recognize_figures(move |name| {
        let mut guard = b.lock().unwrap();

        match name.as_str() {
            "rectangle" => { guard.init(); /*tx.send(()).unwrap();*/ },
            "triangle" => { /* c.close(); */ guard.confirm(); }
            "delete" => { /* c.close(); */ guard.cancel(); }
            _ => {},
        }
    });


    // TESTING
    //confirm_dialog.open(|_,_|{});
    // TESTING
    // let c1 = confirm_dialog.clone();
    // loop {
    //     let b1 = backupper.clone();

    //     match rx.recv() {
    //         Ok(_) => {
    //             c1.open(move |result, is_running| {
    //                 (*(*is_running).lock().unwrap()) = false;
    //                 let mut guard = b1.lock().unwrap();
    //                 match result {
    //                     true => guard.confirm(),
    //                     false => guard.cancel(),
    //                 };
    //             });
    //         },
    //         Err(e) => {
    //             println!("CHANNEL ERROR: {:?}", e);
    //             break;
    //         }
    //     };
    // }

    h.join().unwrap();
}




#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use backupper::{Backupper, BackupperStatus};
    use handle_figure_recognition::recognize_figures;

    #[test]
    fn test_backup_init() {
        let backupper = Arc::new(Mutex::new(Backupper::new()));
        let b = backupper.clone();
        
        let _h = recognize_figures(move |name| {
            if name == "rectangle" {
                let mut guard = b.lock().unwrap();
                guard.init();
                assert_eq!(guard.get_status(), BackupperStatus::WaitingConfirm);
            }
        });

        // Simulate drawing a rectangle
        thread::sleep(Duration::from_millis(100));
    }

    #[test]
    fn test_backup_confirm() {
        let backupper = Arc::new(Mutex::new(Backupper::new()));
        let b = backupper.clone();
        
        let _h = recognize_figures(move |name| {
            if name == "triangle" {
                let mut guard = b.lock().unwrap();
                guard.confirm();
                assert_eq!(guard.get_status(), BackupperStatus::Running);
            }
        });

        // Simulate drawing a triangle
        thread::sleep(Duration::from_millis(100));
    }

    #[test]
    fn test_backup_cancel() {
        let backupper = Arc::new(Mutex::new(Backupper::new()));
        let b = backupper.clone();
        
        let _h = recognize_figures(move |name| {
            if name == "delete" {
                let mut guard = b.lock().unwrap();
                guard.cancel();
                assert_eq!(guard.get_status(), BackupperStatus::Ready);
            }
        });

        // Simulate drawing a delete gesture
        thread::sleep(Duration::from_millis(100));
    }
}


use std::sync::{mpsc::Receiver, Arc, Mutex};
use native_dialog::{MessageDialog, MessageType};
use crate::backupper::Backupper;


pub struct ConfirmDialog {}

impl ConfirmDialog {
    pub fn handle(opener: Receiver<()>, backupper: Arc<Mutex<Backupper>>) {
        loop {
            match opener.recv() {
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
    }

    fn open() -> bool {
        // Mostra una finestra di dialogo con pulsanti "Sì" e "No"
        let dialog = MessageDialog::new()
            .set_title("Backup Confirmation")
            .set_text("Do you want to proceed with the backup?")
            .set_type(MessageType::Warning);
        
        match dialog.show_confirm() {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error displaying dialog: {}", e);
                false
            }
        }
    }
}
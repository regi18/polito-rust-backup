
use native_dialog::{MessageDialog, MessageType};
use crate::backupper::Backupper;

pub fn show_confirmation_dialog() -> bool {
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

    //non si chiude la finestra di dialogo dopo che ho premuo "Sì" o "No"
    //come faccio a chiuderla?
    
}

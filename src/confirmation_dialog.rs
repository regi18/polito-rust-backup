use native_dialog::{MessageDialog, MessageType};

pub struct ConfirmDialog {}

impl ConfirmDialog {
    pub fn open() -> bool {
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
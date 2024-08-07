use native_dialog::{MessageDialog, MessageType};

fn main() {
    // Mostra una finestra di dialogo con pulsanti "Sì" e "No"
    let dialog = MessageDialog::new()
        .set_title("Backup Confirmation")
        .set_text("Do you want to proceed with the backup?")
        .set_type(MessageType::Warning);
    
    match dialog.show_confirm() {
        Ok(true) => {
            std::process::exit(0); // Return 0 for success
        },
        Ok(false) => {
            std::process::exit(1); // Return 1 for user cancel
        },
        Err(_) => {
            std::process::exit(2); // Return 2 for error
        }
    }
}
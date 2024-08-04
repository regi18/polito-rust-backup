
use native_dialog::{MessageDialog, MessageType};
use crate::backupper::Backupper;

//take in ingress the Backupper struct
pub fn show_confirmation_dialog(backupper: &mut Backupper) {
    // Mostra una finestra di dialogo con pulsanti "Sì" e "No"
    let result = MessageDialog::new()
        .set_title("Backup Confirmation")
        .set_text("Do you want to proceed with the backup?")
        .set_type(MessageType::Warning)
        .show_confirm()
        .unwrap();
    
    match result {
        true => backupper.confirm(), //if the user press "Yes" the confirm method is called
        false => backupper.cancel(), //if the user press "No" the cancel method is called
    }
}

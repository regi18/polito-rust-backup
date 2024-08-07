use std::process::{Child, Command};

const BIN_PATH: &str = "./target/debug/confirmation_dialog";

pub struct ConfirmDialog {
    child: Option<Child>,
}

impl ConfirmDialog {
    pub fn new() -> Self {
        Self { 
            child: None,
        }
    }

    pub fn open(&mut self) {
        self.child = Some(Command::new(BIN_PATH).spawn().unwrap());
    }

    pub fn close(&mut self) {
        match &mut self.child {
            Some(c) => { 
                println!("CLOSING PID: {:?}", c.id()); 
                // c.kill().unwrap(); 
            },
            None => {},
        };
    }

    pub fn check_result(&mut self) -> Option<bool> {
        let c = match &mut self.child {
            Some(c) => c,
            None => panic!("Tried to poll but the dialog is not yet open"),
        };

        match c.try_wait() {
            Ok(Some(status)) => {
                let exit_code = status.code().unwrap_or(-1);

                let r = match exit_code {
                    0 => true,
                    _ => false,
                };
                Some(r)
            }
            Ok(None) => None,
            Err(_) => None,
        }
    }
}
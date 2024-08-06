use std::fs;

pub fn print_help() {
    match fs::read_to_string("assets/help.txt") {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("Failed to read help file: {}", e),
    }
}

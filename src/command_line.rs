use std::{fs, process};
use clap::{App, Arg};


pub struct CommandLine {}
impl CommandLine {
    pub fn handle() {
        let matches = App::new("backup_tool")
            .about("A tool for creating backups using a visual command.")
            .arg(Arg::with_name("help")
                .short("h")
                .long("help")
                .help("Prints help information"))
            .get_matches();

        // verify if the help flag is present
        if matches.is_present("help") {
            CommandLine::print_help();
            process::exit(0);
        }
    }

    fn print_help() {
        match fs::read_to_string("assets/help.txt") {
            Ok(content) => println!("{}", content),
            Err(e) => eprintln!("Failed to read help file: {}", e),
        }
    }
}

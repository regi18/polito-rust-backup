use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

// Config struct holds to data from the `[config]` section.
#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub backup_source: String,
    pub backup_dest: String,
    pub file_types: Vec<String>
}

impl Config {
    pub fn new() -> Self {
        let filename = "Config.toml";

        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Could not read file `{}`", filename);
                exit(1);
            }
        };

        let config: Config = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Unable to load data from `{}`", filename);
                exit(1);
            }
        };

        return config;
    }
}
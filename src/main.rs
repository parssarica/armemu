pub mod errormsg;
mod parse_toml;

use errormsg::*;
use std::process::exit;

fn main() {
    let config = match parse_toml::parse_file() {
        Ok(n) => n,
        Err(e) => {
            match e {
                parse_toml::ErrorType::FileNotFound => fail_normal("File 'config.toml' not found."),
                parse_toml::ErrorType::Other(x) => fail_normal("Invalid 'config.toml' file."),
            }
            exit(1);
        }
    };
    println!("Parsed: {:#?}", config);
}

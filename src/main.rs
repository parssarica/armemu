pub mod errormsg;
pub mod instruction_parser;
mod parse_toml;
pub mod registers;
mod tests;

use errormsg::*;
use std::process::exit;

fn main() {
    let config = match parse_toml::parse_file() {
        Ok(n) => n,
        Err(e) => {
            match e {
                parse_toml::ErrorType::FileNotFound => fail_normal("File 'config.toml' not found."),
                parse_toml::ErrorType::Other(_) => fail_normal("Invalid 'config.toml' file."),
            }
            exit(1);
        }
    };
    let registers = registers::create_registers();
    println!("Parsed: {:#?}", config);
}

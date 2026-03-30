pub mod dbgview;
pub mod errormsg;
mod executer;
pub mod instruction_parser;
pub mod instructions;
mod parse_toml;
pub mod registers;
mod tests;

use errormsg::*;
use std::fs;
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

    let mut memory: Vec<u8> = vec![0; parse_toml::parse_memory(&config.memory)];
    let mut registers = registers::create_registers();

    let Ok(file_utf8) = fs::read(&config.file) else {
        fail_normal(&format!("Input file '{}' is invalid.", config.file));
        exit(1);
    };

    let Ok(file) = String::from_utf8(file_utf8) else {
        fail_normal(&format!("Input file '{}' is invalid.", config.file));
        exit(1);
    };

    let Some(mut code) = instruction_parser::parse_file(&registers, &file) else {
        fail_normal(&format!("Input file '{}' is invalid.", config.file));
        exit(1);
    };

    executer::execute(&mut code, &mut registers, &mut memory);
}

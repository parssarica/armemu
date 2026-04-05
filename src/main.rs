pub mod dbgview;
pub mod errormsg;
mod executer;
pub mod instruction_parser;
pub mod instructions;
mod parse_toml;
pub mod registers;
pub mod syscalls;
mod tests;

use capstone::prelude::*;
use errormsg::*;
use object::{Object, ObjectSegment};
use registers::*;
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

    let mut memory: Vec<u8> = vec![0; parse_toml::parse_memory(&config.config.memory)];
    let mut registers = create_registers();
    let file_utf8;

    if config.config.code.is_some() && config.config.binary.is_some() {
        fail_normal(
            "You can only provide the code or the binary field in the TOML file, not both of them.",
        );
        exit(1);
    } else if config.config.code.is_none() && config.config.binary.is_none() {
        fail_normal("You should provide the code field or the binary field in the TOML file.");
        exit(1);
    }

    let mut entry_point = 0;
    let file;
    let cs;
    match config.config.code {
        Some(ref n) => {
            file = fs::read(n).unwrap_or_else(|_| {
                fail_normal(&format!(
                    "Input file '{}' does not exist.",
                    config.config.code.as_ref().unwrap()
                ));
                exit(1);
            });

            cs = None;
        }
        None => {
            let bin_file = config.config.binary.as_ref().unwrap();

            file_utf8 = fs::read(bin_file).unwrap_or_else(|_| {
                fail_normal(&format!(
                    "Input file '{}' does not exist.",
                    config.config.binary.as_ref().unwrap()
                ));
                exit(1);
            });

            let f = object::File::parse(&*file_utf8).unwrap_or_else(|_| {
                fail_normal("Binary file is invalid.");
                exit(1);
            });

            let base_address = f
                .segments()
                .map(|seg| seg.address())
                .next()
                .unwrap_or_else(|| {
                    fail_normal("Binary file does not have a base address.");
                    exit(1);
                });
            cs = Some(
                Capstone::new()
                    .arm64()
                    .mode(arch::arm64::ArchMode::Arm)
                    .build()
                    .unwrap(),
            );
            entry_point = f.entry();
            file = (&file_utf8[((entry_point - base_address) as usize)..]).to_vec();

            set_register_value(&mut registers, "PC", RegisterValue::Val64(entry_point));

            let mut i = 0;
            for n in &file_utf8 {
                *(memory.get_mut((base_address + i) as usize).unwrap_or_else(|| {
                    fail_normal(
                        &format!("Insufficient memory for loading the binary. Needing at least {} bytes.",
                        (base_address as usize) + file_utf8.len()),
                    );
                    exit(1);
                })) = *n;
                i += 1;
            }
        }
    };

    let labels = match config.config.code {
        Some(ref n) => instruction_parser::parse_labels(n),
        None => Vec::new(),
    };

    let mut code = None;
    match config.config.binary {
        Some(_) => (),
        None => {
            code = Some(
                instruction_parser::parse_file(&registers, str::from_utf8(&file).unwrap(), &labels)
                    .unwrap_or_else(|| {
                        fail_normal(&format!(
                            "Input file '{}' is invalid.",
                            match config.config.code {
                                Some(c) => c,
                                None => config.config.binary.unwrap(),
                            }
                        ));
                        exit(1);
                    }),
            );
        }
    }

    let debug_mode_on = match config.debugview {
        Some(n) => n.debugmode,
        None => true,
    };

    match cs {
        Some(ref n) => executer::execute_disasm(
            &file,
            n,
            &mut registers,
            &mut memory,
            debug_mode_on,
            entry_point as usize,
        ),
        None => executer::execute_normal(
            code.as_mut().unwrap(),
            &mut registers,
            &mut memory,
            debug_mode_on,
        ),
    }
}

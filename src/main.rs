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
use object::read::elf::{ElfFile, ProgramHeader};
use object::{elf, Endianness};
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

    let memory_size = parse_toml::parse_memory(&config.config.memory);
    let mut memory: Vec<u8> = vec![0; memory_size];
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

            let f =
                ElfFile::<elf::FileHeader64<Endianness>>::parse(&*file_utf8).unwrap_or_else(|_| {
                    fail_normal("Only ELF is supported.");
                    exit(1);
                });
            let endian = f.endian();

            let base_address = f
                .elf_program_headers()
                .iter()
                .filter(|ph| ph.p_type(endian) == elf::PT_LOAD)
                .map(|ph| ph.p_vaddr(endian))
                .min()
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

            entry_point = f.raw_header().e_entry.get(endian);
            file = (&file_utf8[((entry_point - base_address) as usize)..]).to_vec();

            set_register_value(&mut registers, "PC", RegisterValue::Val64(entry_point));

            for ph in f.elf_program_headers() {
                let p_type = ph.p_type(endian);
                match p_type {
                    elf::PT_LOAD => {
                        let virt_addr = ph.p_vaddr(endian);
                        let offset = ph.p_offset(endian);
                        let align = ph.p_align(endian);
                        let filesz = ph.p_filesz(endian);
                        let memsz = ph.p_memsz(endian);

                        if virt_addr % align != offset % align {
                            fail_normal("ELF binary is invalid.");
                            exit(1);
                        }

                        let addr = virt_addr % align;
                        if filesz + addr >= memory_size as u64 {
                            fail_normal("Insufficient memory.");
                            exit(1);
                        }

                        if memsz > filesz {
                            for i in 0..memsz {
                                memory[(virt_addr + i) as usize] = 0;
                            }
                        } else {
                            for i in 0..filesz {
                                memory[(virt_addr + i) as usize] = file_utf8[(addr + i) as usize];
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    };

    let labels = match config.config.code {
        Some(_) => instruction_parser::parse_labels(str::from_utf8(&file).unwrap_or_else(|_| {
            fail_normal("File contains non UTF-8 characters.");
            exit(1);
        })),
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

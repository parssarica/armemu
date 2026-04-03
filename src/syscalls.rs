use crate::registers::*;
use std::io::{self, Read, Write};
use std::process::exit;

pub fn sys_write(registers: &mut Vec<Register>, memory: &Vec<u8>) {
    let _fd = get_register_value(registers, "X0").unwrap();
    let addr = match get_register_value(registers, "X1").unwrap() {
        RegisterValue::Val32(_) => unreachable!(),
        RegisterValue::Val64(n) => n as usize,
    };
    let length = match get_register_value(registers, "X2").unwrap() {
        RegisterValue::Val32(_) => unreachable!(),
        RegisterValue::Val64(n) => n as usize,
    };
    let s = &memory[addr..(addr + length)];
    let mut stdout = io::stdout();

    stdout.write_all(s).expect("Failed to write to stdout");

    set_register_value(
        registers,
        "X0",
        get_register_value(registers, "X2").unwrap(),
    );
}

pub fn sys_read(registers: &mut Vec<Register>, memory: &mut Vec<u8>) {
    let _fd = get_register_value(registers, "X0").unwrap();
    let addr = match get_register_value(registers, "X1").unwrap() {
        RegisterValue::Val32(_) => unreachable!(),
        RegisterValue::Val64(n) => n as usize,
    };
    let length = match get_register_value(registers, "X2").unwrap() {
        RegisterValue::Val32(_) => unreachable!(),
        RegisterValue::Val64(n) => n as usize,
    };
    let s = &mut memory[addr..(addr + length)];
    let mut stdin = io::stdin();

    let bytes_read = stdin.read(s).expect("Failed to read from stdin");

    set_register_value(registers, "X0", RegisterValue::Val64(bytes_read as u64));
}

pub fn sys_exit(registers: &Vec<Register>) {
    let return_code = get_register_value(registers, "X0").unwrap().convert_32() as i32;

    exit(return_code);
}

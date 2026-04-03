use crate::registers::*;
use std::io::{self, Write};

pub fn sys_write(registers: &Vec<Register>, memory: &Vec<u8>) {
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
}

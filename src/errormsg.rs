use crate::registers::*;

pub fn fail_normal(msg: &str) {
    eprintln!("\x1b[91mERROR:\x1b[0m {}", msg);
}

pub fn fail_error(msg: Box<dyn std::error::Error>) {
    eprintln!("\x1b[91mERROR:\x1b[0m {:?}", msg);
}

pub fn fail(registers: &Vec<Register>, msg: &str) {
    eprintln!(
        "\x1b[91mError:\x1b[0m PC is {:#x}: {}",
        match get_register_value(registers, "PC").unwrap() {
            RegisterValue::Val64(n) => n,
            _ => unreachable!(),
        },
        msg
    );
}

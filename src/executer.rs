use crate::{dbgview::*, errormsg::*, instruction_parser::*, instructions::*, registers::*};
use std::process::exit;

pub fn execute(code: &Vec<Instruction>, registers: &mut Vec<Register>, _memory: &mut Vec<u8>) {
    let mut ins_output: Result<(), String> = Ok(());
    let mut ins;
    let mut last_msg = String::new();

    loop {
        last_msg = debug_view(registers, code, &last_msg);

        ins = code
            .get(
                (match get_register_value(registers, "PC") {
                    Some(RegisterValue::Val64(n)) => n,
                    _ => return,
                }) as usize,
            )
            .unwrap_or_else(|| {
                fail(
                    registers,
                    "No instruction or system call to end the program.",
                );
                exit(1);
            });

        match ins.name.to_lowercase().as_str() {
            "mov" => mov(registers, ins, &mut ins_output),
            "add" => add(registers, ins, &mut ins_output),
            "sub" => sub(registers, ins, &mut ins_output),
            "mul" => mul(registers, ins, &mut ins_output),
            "and" => and(registers, ins, &mut ins_output),
            _ => {
                fail(
                    registers,
                    &format!("Unknown instruction '{}'", ins.name.as_str().to_lowercase()),
                );
                exit(1);
            }
        }

        match ins_output {
            Ok(_) => (),
            Err(e) => {
                fail(registers, &e);
                exit(1);
            }
        }

        set_register_value(
            registers,
            "PC",
            RegisterValue::Val64(
                (match get_register_value(registers, "PC") {
                    Some(RegisterValue::Val64(n)) => n,
                    _ => unreachable!(),
                }) + 1,
            ),
        );
    }
}

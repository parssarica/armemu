use crate::errormsg::*;
use crate::instruction_parser::*;
use crate::instructions::*;
use crate::registers::*;
use std::process::exit;

pub fn execute(code: &Vec<Instruction>, registers: &mut Vec<Register>, _memory: &mut Vec<u8>) {
    let mut ins_output: Result<(), String> = Ok(());
    for ins in code {
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
    }
}

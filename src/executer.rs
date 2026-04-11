use crate::{dbgview::*, errormsg::*, instruction_parser::*, instructions::*, registers::*};
use capstone::prelude::*;
use std::process::exit;

fn get_last_operand(ins: &Instruction) -> Option<&Operand> {
    if ins.op4.is_some() {
        return ins.op4.as_ref();
    }

    if ins.op3.is_some() {
        return ins.op3.as_ref();
    }

    if ins.op2.is_some() {
        return ins.op2.as_ref();
    }

    if ins.op1.is_some() {
        return ins.op1.as_ref();
    }

    None
}

pub fn exec_ins(ins: &mut Instruction, registers: &mut Vec<Register>, mut memory: &mut Vec<u8>) {
    let mut new_op: Option<Operand> = None;
    let mut old_val: Option<RegisterValue> = None;
    let mut register_barrel_shifter: Option<String> = None;
    let converted: Instructions;
    let ins_org = ins.clone();
    let ins_output: Result<(), String> = Ok(());

    match &ins.barrelshifter {
        None => (),
        Some(_) => match get_last_operand(&ins) {
            Some(_) => (),
            None => {
                fail(
                    registers,
                    "Barrel shifter can't work if the second operand doesn't exist.",
                );
                exit(1);
            }
        },
    }

    if ins.barrelshifter.is_some() {
        match get_last_operand(&ins) {
            None => (),
            Some(n) => match n {
                Operand::OperandRegister(n) => {
                    new_op = None;
                    register_barrel_shifter = Some(n.clone());
                    old_val = match get_register_value(registers, &n) {
                        Some(k) => Some(k),
                        None => {
                            fail(registers, &format!("Register '{}' not found.", &n));
                            exit(1);
                        }
                    };
                    let newval = ins
                        .barrelshifter
                        .as_ref()
                        .unwrap()
                        .parse_val(registers, get_register_value(registers, &n).unwrap())
                        .unwrap_or_else(|| {
                            fail(registers, "Invalid barrel shifter.");
                            exit(1);
                        });
                    set_register_value(registers, &n, newval);
                }
                Operand::OperandNumber(n) => {
                    new_op = Some(Operand::OperandNumber(
                        ins.barrelshifter
                            .as_ref()
                            .unwrap()
                            .parse_val(registers, *n)
                            .unwrap_or_else(|| {
                                fail(registers, "Invalid barrel shifter.");
                                exit(1);
                            }),
                    ));
                }
                _ => {
                    fail(registers, "Barrel shifter can't work on a memory address.");
                    exit(1);
                }
            },
        }
    }

    if new_op.as_ref().is_some() {
        ins.op2 = new_op.clone();
    }

    if ins_org.name.to_lowercase() == "movk" {
        movk(registers, &ins_org);
        return;
    }

    converted = convert_ins(&ins, &registers).unwrap_or_else(|n| {
        fail(registers, &format!("{}", n));
        exit(1);
    });

    match converted {
        Instructions::Mov { ref op1, op2 } => mov(registers, op1, op2),
        Instructions::Add {
            ref op1,
            ref op2,
            op3,
        } => add(registers, op1, op2, op3),
        Instructions::Sub {
            ref op1,
            ref op2,
            op3,
        } => sub(registers, op1, op2, op3),
        Instructions::Mul {
            ref op1,
            ref op2,
            op3,
        } => mul(registers, op1, op2, op3),
        Instructions::And {
            ref op1,
            ref op2,
            op3,
        } => and(registers, op1, op2, op3),
        Instructions::Ldr { ref op1, ref op2 } => {
            ldr(registers, op1, op2, memory).unwrap_or_else(|n| {
                fail(registers, &n);
                exit(1);
            })
        }
        Instructions::Str { ref op1, op2 } => {
            str(registers, op1, op2, &mut memory).unwrap_or_else(|n| {
                fail(registers, &n);
                exit(1);
            })
        }
        Instructions::Cmp { ref op1, ref op2 } => cmp(registers, op1, op2),
        Instructions::B { ref op1 } => b(registers, op1),
        Instructions::Beq { ref op1 } => beq(registers, op1),
        Instructions::Bne { ref op1 } => bne(registers, op1),
        Instructions::Bgt { ref op1 } => bgt(registers, op1),
        Instructions::Blt { ref op1 } => blt(registers, op1),
        Instructions::Bge { ref op1 } => bge(registers, op1),
        Instructions::Svc { .. } => svc(registers, &mut memory).unwrap_or_else(|n| {
            fail(registers, &n);
            exit(1);
        }),
        Instructions::Adds {
            ref op1,
            ref op2,
            ref op3,
        } => adds(registers, op1, op2, op3),
        Instructions::Subs {
            ref op1,
            ref op2,
            ref op3,
        } => subs(registers, op1, op2, op3),
        Instructions::Adr { ref op1, ref op2 } => adr(registers, op1, op2),
        Instructions::Adrp { ref op1, ref op2 } => adrp(registers, op1, op2),
        Instructions::Orr {
            ref op1,
            ref op2,
            ref op3,
        } => orr(registers, op1, op2, op3),
        Instructions::Eor {
            ref op1,
            ref op2,
            ref op3,
        } => eor(registers, op1, op2, op3),
        Instructions::Eon {
            ref op1,
            ref op2,
            ref op3,
        } => eon(registers, op1, op2, op3),
        Instructions::Bic {
            ref op1,
            ref op2,
            ref op3,
        } => bic(registers, op1, op2, op3),
        Instructions::Lsl {
            ref op1,
            ref op2,
            ref op3,
        } => lsl(registers, op1, op2, op3),
        Instructions::Lsr {
            ref op1,
            ref op2,
            ref op3,
        } => lsr(registers, op1, op2, op3),
        Instructions::Asr {
            ref op1,
            ref op2,
            ref op3,
        } => asr(registers, op1, op2, op3),
        Instructions::Ror {
            ref op1,
            ref op2,
            ref op3,
        } => ror(registers, op1, op2, op3),
        Instructions::Ubfx {
            ref op1,
            ref op2,
            ref op3,
            ref op4,
        } => ubfx(registers, op1, op2, op3, op4),
        Instructions::MoreThanOneByte => unreachable!(),
    }

    match ins_output {
        Ok(_) => (),
        Err(e) => {
            fail(registers, &e);
            exit(1);
        }
    }

    match old_val {
        Some(n) => {
            set_register_value(registers, &register_barrel_shifter.unwrap(), n);
        }
        None => (),
    }
}

pub fn execute_normal(
    code: &mut Vec<Instruction>,
    registers: &mut Vec<Register>,
    memory: &mut Vec<u8>,
    debug_mode_on: bool,
) {
    let mut last_msg = String::new();
    let mut i = 0;
    let mut pc;
    let mut is_instruction;

    loop {
        pc = get_register_value(registers, "PC").unwrap().convert_64();
        is_instruction = !code
            .get(pc as usize)
            .unwrap_or_else(|| {
                fail(
                    registers,
                    "No instruction or system call to end the program.",
                );
                exit(1);
            })
            .name
            .eq_ignore_ascii_case("morethanonebyte");
        if i == 0 || is_instruction {
            if debug_mode_on {
                last_msg = debug_view_normal(registers, &code[(pc as usize)..], &last_msg, &memory);
            }
        }

        if is_instruction {
            let ins = code.get_mut(pc as usize).unwrap_or_else(|| {
                fail(
                    registers,
                    "No instruction or system call to end the program.",
                );
                exit(1);
            });

            exec_ins(ins, registers, memory);
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
        i = 1;
    }
}

pub fn execute_disasm(
    file: &[u8],
    cs: &Capstone,
    registers: &mut Vec<Register>,
    memory: &mut Vec<u8>,
    debug_mode_on: bool,
    entry_point: usize,
) {
    let mut last_msg = String::new();
    let mut ins;
    let mut pc;
    let mut disassembled_ins;
    let mut disassembled;
    let mut disassembled_multi;
    let disasm_point =
        (get_register_value(registers, "PC").unwrap().convert_64() as usize - entry_point) as u64;

    loop {
        pc = get_register_value(registers, "PC").unwrap().convert_64() as usize - entry_point;
        if debug_mode_on {
            last_msg = debug_view_disasm(
                registers,
                &file[pc..(pc + 12)],
                &last_msg,
                &memory,
                cs,
                disasm_point,
            );
        }

        if pc + 4 >= file.len() {
            fail_normal("No instruction or syscall to end the program");
            exit(1);
        }
        ins = &file[pc..(pc + 4)];
        // ins_reversed = ins.iter().rev().cloned().collect();

        disassembled_multi = cs.disasm_all(&ins, disasm_point).unwrap_or_else(|_| {
            fail_normal(&format!("Invalid bytes at {:#08X}", pc));
            exit(1);
        });
        disassembled_ins = disassembled_multi.first().unwrap_or_else(|| {
            fail_normal(&format!("Invalid bytes at {:#08X}", pc));
            exit(1);
        });

        disassembled = format!("{}", disassembled_ins.mnemonic().unwrap());
        if let Some(n) = disassembled_ins.op_str() {
            disassembled.push_str(&format!(" {}", n));
        }

        exec_ins(
            &mut parse_instruction(&disassembled, registers, &Vec::new(), 0).unwrap(),
            registers,
            memory,
        );

        set_register_value(
            registers,
            "PC",
            RegisterValue::Val64(
                (match get_register_value(registers, "PC") {
                    Some(RegisterValue::Val64(n)) => n,
                    _ => unreachable!(),
                }) + 4,
            ),
        );
    }
}

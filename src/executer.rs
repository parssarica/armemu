use crate::{dbgview::*, errormsg::*, instruction_parser::*, instructions::*, registers::*};
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

pub fn execute(
    code: &mut Vec<Instruction>,
    registers: &mut Vec<Register>,
    mut memory: &mut Vec<u8>,
) {
    let ins_output: Result<(), String> = Ok(());
    let mut ins;
    let mut last_msg = String::new();
    let mut new_op: Option<Operand> = None;
    let mut old_val: Option<RegisterValue>;
    let mut register_barrel_shifter: Option<String>;
    let mut converted: Instructions;

    loop {
        last_msg = debug_view(registers, code, &last_msg, &memory);
        register_barrel_shifter = None;
        old_val = None;

        ins = code
            .get_mut(
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

        converted = convert_ins(&ins).unwrap_or_else(|n| {
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
            Instructions::Ldr { ref op1, ref op2 } => ldr(registers, op1, op2, memory)
                .unwrap_or_else(|n| {
                    fail(registers, &n);
                    exit(1);
                }),
            Instructions::Str { ref op1, op2 } => str(registers, op1, op2, &mut memory)
                .unwrap_or_else(|n| {
                    fail(registers, &n);
                    exit(1);
                }),
            Instructions::Cmp { ref op1, ref op2 } => cmp(registers, op1, op2),
            Instructions::B { ref op1 } => b(registers, op1),
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

        match old_val {
            Some(n) => {
                set_register_value(registers, &register_barrel_shifter.unwrap(), n);
            }
            None => (),
        }
    }
}

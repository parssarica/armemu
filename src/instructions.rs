use crate::errormsg::*;
use crate::instruction_parser::*;
use crate::registers::*;
use crate::syscalls::*;
use std::process::exit;

pub enum OperandType {
    Register,
    Immediate,
    MemoryAddress,
    RegImm,
    RegMem,
    ImmMem,
    Triple,
}

pub enum Instructions {
    MoreThanOneByte,
    Mov {
        op1: String,
        op2: Operand,
    },
    Add {
        op1: String,
        op2: String,
        op3: Operand,
    },
    Sub {
        op1: String,
        op2: String,
        op3: Operand,
    },
    Mul {
        op1: String,
        op2: String,
        op3: Operand,
    },
    And {
        op1: String,
        op2: String,
        op3: Operand,
    },
    Ldr {
        op1: String,
        op2: Operand,
    },
    Str {
        op1: String,
        op2: MemoryAddress,
    },
    Cmp {
        op1: String,
        op2: Operand,
    },
    B {
        op1: Operand,
    },
    Beq {
        op1: Operand,
    },
    Bne {
        op1: Operand,
    },
    Bgt {
        op1: Operand,
    },
    Blt {
        op1: Operand,
    },
    Bge {
        op1: Operand,
    },
    Svc {
        op1: Operand,
    },
    Adds {
        op1: String,
        op2: String,
        op3: Operand,
    },
    Subs {
        op1: String,
        op2: String,
        op3: Operand,
    },
    Adr {
        op1: String,
        op2: Operand,
    },
    Adrp {
        op1: String,
        op2: Operand,
    },
    Orr {
        op1: String,
        op2: String,
        op3: Operand,
    },
}

pub fn convert_ins(ins: &Instruction, registers: &Vec<Register>) -> Result<Instructions, String> {
    match ins.name.to_lowercase().replace(".", "").as_str() {
        "mov" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            None,
            registers,
        )?,
        "add" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            registers,
        )?,
        "sub" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            registers,
        )?,
        "mul" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            registers,
        )?,
        "and" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            registers,
        )?,
        "ldr" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::ImmMem),
            None,
            None,
            registers,
        )?,
        "str" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::MemoryAddress),
            None,
            None,
            registers,
        )?,
        "cmp" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            None,
            registers,
        )?,
        "b" => operand_check(ins, Some(OperandType::RegImm), None, None, None, registers)?,
        "beq" => operand_check(ins, Some(OperandType::RegImm), None, None, None, registers)?,
        "bne" => operand_check(ins, Some(OperandType::RegImm), None, None, None, registers)?,
        "bgt" => operand_check(ins, Some(OperandType::RegImm), None, None, None, registers)?,
        "blt" => operand_check(ins, Some(OperandType::RegImm), None, None, None, registers)?,
        "bge" => operand_check(ins, Some(OperandType::RegImm), None, None, None, registers)?,
        "svc" => operand_check(
            ins,
            Some(OperandType::Immediate),
            None,
            None,
            None,
            registers,
        )?,
        "adds" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            registers,
        )?,
        "subs" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            registers,
        )?,
        "adr" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Immediate),
            None,
            None,
            registers,
        )?,
        "adrp" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            None,
            registers,
        )?,
        "orr" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            registers,
        )?,
        _ => return Err(format!("Unknown instruction: {}", ins.name.as_str())),
    }

    Ok(
        match ins.name.to_lowercase().as_str().replace(".", "").as_str() {
            "morethanonebyte" => Instructions::MoreThanOneByte,
            "mov" => Instructions::Mov {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: ins.op2.as_ref().unwrap().clone(),
            },
            "add" => Instructions::Add {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op3: ins.op3.as_ref().unwrap().clone(),
            },
            "sub" => Instructions::Sub {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op3: ins.op3.as_ref().unwrap().clone(),
            },
            "mul" => Instructions::Mul {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op3: ins.op3.as_ref().unwrap().clone(),
            },
            "and" => Instructions::And {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op3: ins.op3.as_ref().unwrap().clone(),
            },
            "ldr" => Instructions::Ldr {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: ins.op2.as_ref().unwrap().clone(),
            },
            "str" => Instructions::Str {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandAddress(n) => n.clone(),
                    _ => unreachable!(),
                },
            },
            "cmp" => Instructions::Cmp {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: ins.op2.as_ref().unwrap().clone(),
            },
            "b" => Instructions::B {
                op1: ins.op1.as_ref().unwrap().clone(),
            },
            "beq" => Instructions::Beq {
                op1: ins.op1.as_ref().unwrap().clone(),
            },
            "bne" => Instructions::Bne {
                op1: ins.op1.as_ref().unwrap().clone(),
            },
            "bgt" => Instructions::Bgt {
                op1: ins.op1.as_ref().unwrap().clone(),
            },
            "blt" => Instructions::Blt {
                op1: ins.op1.as_ref().unwrap().clone(),
            },
            "bge" => Instructions::Bge {
                op1: ins.op1.as_ref().unwrap().clone(),
            },
            "svc" => Instructions::Svc {
                op1: ins.op1.as_ref().unwrap().clone(),
            },
            "adds" => Instructions::Adds {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op3: ins.op3.as_ref().unwrap().clone(),
            },
            "subs" => Instructions::Subs {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op3: ins.op3.as_ref().unwrap().clone(),
            },
            "adr" => Instructions::Adr {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: ins.op2.as_ref().unwrap().clone(),
            },
            "adrp" => Instructions::Adrp {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: ins.op2.as_ref().unwrap().clone(),
            },
            "orr" => Instructions::Orr {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op3: ins.op3.as_ref().unwrap().clone(),
            },
            _ => unreachable!(),
        },
    )
}

pub fn group_couple(s: &str) -> Vec<String> {
    let mut j = String::new();
    let mut out = Vec::new();
    let mut k = 0;

    for i in 0..s.len() {
        if k < 2 {
            j.push(s.chars().nth(i as usize).unwrap());
        } else {
            out.push(j);
            j = String::from(s.chars().nth(i as usize).unwrap());
            k = 0;
        }

        k += 1;
    }

    if j != "" {
        out.push(j);
    }

    out
}

pub fn operand_check(
    instruction: &Instruction,
    op1_type: Option<OperandType>,
    op2_type: Option<OperandType>,
    op3_type: Option<OperandType>,
    op4_type: Option<OperandType>,
    registers: &Vec<Register>,
) -> Result<(), String> {
    let mut operand_count = 0;
    operand_count += match op1_type {
        Some(_) => 1,
        None => 0,
    };

    operand_count += match op2_type {
        Some(_) => 1,
        None => 0,
    };

    operand_count += match op3_type {
        Some(_) => 1,
        None => 0,
    };

    operand_count += match op4_type {
        Some(_) => 1,
        None => 0,
    };

    if operand_count != instruction.operand_count {
        return Err("Invalid operand count".to_string());
    }

    if operand_count > 0 {
        match instruction.op1.as_ref().unwrap() {
            Operand::OperandRegister(n) => match op1_type.unwrap() {
                OperandType::Immediate | OperandType::MemoryAddress => {
                    return Err("Invalid type in operand 1".to_string())
                }
                _ => {
                    if get_register_value(registers, n).is_none() {
                        return Err("Invalid register in operand 1".to_string());
                    }
                }
            },
            Operand::OperandNumber(_) => match op1_type.unwrap() {
                OperandType::Register | OperandType::MemoryAddress => {
                    return Err("Invalid type in operand 1".to_string())
                }
                _ => (),
            },
            Operand::OperandAddress(_) => match op1_type.unwrap() {
                OperandType::Register | OperandType::Immediate => {
                    return Err("Invalid type in operand 1".to_string())
                }
                _ => (),
            },
        }
    }

    if operand_count > 1 {
        match instruction.op2.as_ref().unwrap() {
            Operand::OperandRegister(n) => match op2_type.unwrap() {
                OperandType::Immediate | OperandType::MemoryAddress => {
                    return Err("Invalid type in operand 2".to_string())
                }
                _ => {
                    if get_register_value(registers, n).is_none() {
                        return Err("Invalid register in operand 2".to_string());
                    }
                }
            },
            Operand::OperandNumber(_) => match op2_type.unwrap() {
                OperandType::Register | OperandType::MemoryAddress => {
                    return Err("Invalid type in operand 2".to_string())
                }
                _ => (),
            },
            Operand::OperandAddress(_) => match op2_type.unwrap() {
                OperandType::Register | OperandType::Immediate => {
                    return Err("Invalid type in operand 2".to_string())
                }
                _ => (),
            },
        }
    }

    if operand_count > 2 {
        match instruction.op3.as_ref().unwrap() {
            Operand::OperandRegister(n) => match op3_type.unwrap() {
                OperandType::Immediate | OperandType::MemoryAddress => {
                    return Err("Invalid type in operand 3".to_string())
                }
                _ => {
                    if get_register_value(registers, n).is_none() {
                        return Err("Invalid register in operand 3".to_string());
                    }
                }
            },
            Operand::OperandNumber(_) => match op3_type.unwrap() {
                OperandType::Register | OperandType::MemoryAddress => {
                    return Err("Invalid type in operand 3".to_string())
                }
                _ => (),
            },
            Operand::OperandAddress(_) => match op3_type.unwrap() {
                OperandType::Register | OperandType::Immediate => {
                    return Err("Invalid type in operand 3".to_string())
                }
                _ => (),
            },
        }
    }

    if operand_count > 3 {
        match instruction.op4.as_ref().unwrap() {
            Operand::OperandRegister(n) => match op4_type.unwrap() {
                OperandType::Immediate | OperandType::MemoryAddress => {
                    return Err("Invalid type in operand 4".to_string())
                }
                _ => {
                    if get_register_value(registers, n).is_none() {
                        return Err("Invalid register in operand 4".to_string());
                    }
                }
            },
            Operand::OperandNumber(_) => match op4_type.unwrap() {
                OperandType::Register | OperandType::MemoryAddress => {
                    return Err("Invalid type in operand 4".to_string())
                }
                _ => (),
            },
            Operand::OperandAddress(_) => match op4_type.unwrap() {
                OperandType::Register | OperandType::Immediate => {
                    return Err("Invalid type in operand 4".to_string())
                }
                _ => (),
            },
        }
    }

    Ok(())
}

pub fn mov(registers: &mut Vec<Register>, op1: &str, op2: Operand) {
    let op2_val = match op2 {
        Operand::OperandRegister(ref n) => get_register_value(registers, n).unwrap(),
        Operand::OperandNumber(n) => n,
        _ => unreachable!(),
    };

    set_register_value(registers, op1, op2_val);
}

pub fn add(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: Operand) {
    set_register_value(
        registers,
        op1,
        get_register_value(registers, op2).unwrap() + op3.convert_reg_val(registers).unwrap(),
    );
}

pub fn sub(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: Operand) {
    set_register_value(
        registers,
        op1,
        get_register_value(registers, op2).unwrap() - op3.convert_reg_val(registers).unwrap(),
    );
}

pub fn mul(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: Operand) {
    set_register_value(
        registers,
        op1,
        get_register_value(registers, op2).unwrap() * op3.convert_reg_val(registers).unwrap(),
    );
}

pub fn and(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: Operand) {
    set_register_value(
        registers,
        op1,
        get_register_value(registers, op2).unwrap() & op3.convert_reg_val(registers).unwrap(),
    );
}

pub fn ldr(
    registers: &mut Vec<Register>,
    op1: &str,
    op2: &Operand,
    memory: &Vec<u8>,
) -> Result<(), String> {
    let mem_addr_obj = match op2 {
        Operand::OperandAddress(n) => {
            n.change_reg_preindex(registers);
            n
        }
        Operand::OperandNumber(n) => {
            set_register_value(
                registers,
                op1,
                get_register_value(registers, "PC").unwrap() + RegisterValue::Val64(4) + *n,
            );
            return Ok(());
        }
        _ => unreachable!(),
    };

    let addr = mem_addr_obj.get_addr(registers).convert_64();
    let mut bytes = Vec::new();
    let is_32_bit = match op1.chars().nth(0).unwrap() {
        'W' => true,
        _ => false,
    };

    for i in 0..(match is_32_bit {
        true => 4,
        false => 8,
    }) {
        bytes.push(match memory.get((addr + i) as usize) {
            Some(n) => n,
            None => {
                return Err(String::from("Invalid memory address"));
            }
        });
    }

    bytes.reverse();

    if is_32_bit {
        set_register_value(
            registers,
            op1,
            match u32::from_str_radix(
                &bytes
                    .iter()
                    .map(|x: &&u8| format!("{:X}", x))
                    .collect::<Vec<String>>()
                    .concat(),
                16,
            ) {
                Ok(n) => RegisterValue::Val32(n),
                Err(_) => {
                    return Err("Problem when parsing data from memory".to_string());
                }
            },
        );
    } else {
        set_register_value(
            registers,
            op1,
            match u64::from_str_radix(
                &bytes
                    .iter()
                    .map(|x: &&u8| format!("{:X}", x))
                    .collect::<Vec<String>>()
                    .concat(),
                16,
            ) {
                Ok(n) => RegisterValue::Val64(n),
                Err(_) => {
                    return Err("Problem when parsing data from memory".to_string());
                }
            },
        );
    }

    mem_addr_obj.change_reg_postindex(registers);

    Ok(())
}

pub fn str(
    registers: &mut Vec<Register>,
    op1: &str,
    op2: MemoryAddress,
    memory: &mut Vec<u8>,
) -> Result<(), String> {
    op2.change_reg_preindex(registers);

    let addr = op2.get_addr(registers).convert_64();

    let bytes: Vec<String> =
        group_couple(format!("{:016X}", get_register_value(registers, op1).unwrap()).as_str())
            .iter()
            .rev()
            .cloned()
            .collect();
    let mut j = 0;

    if (addr + (bytes.len() as u64) - 1) as usize >= memory.len() {
        return Err("Invalid memory address".to_string());
    }

    for _ in &bytes {
        memory[(addr + j) as usize] =
            u8::from_str_radix(bytes.get(j as usize).unwrap(), 16).unwrap();
        j += 1;
    }

    op2.change_reg_postindex(registers);

    Ok(())
}

pub fn cmp(registers: &mut Vec<Register>, op1: &str, op2: &Operand) {
    let op1_val = get_register_value(registers, op1).unwrap().convert_64() as i64;
    let op2_val = op2.convert_reg_val(registers).unwrap().convert_64() as i64;
    let subtraction = op1_val - op2_val;

    if subtraction < 0 {
        set_flag(registers, "N", true);
    } else {
        set_flag(registers, "N", false);
    }

    if subtraction == 0 {
        set_flag(registers, "Z", true);
    } else {
        set_flag(registers, "Z", false);
    }

    if op1_val >= op2_val {
        set_flag(registers, "C", true);
    } else {
        set_flag(registers, "C", false);
    }

    if (op1_val > 0 && op2_val > 0 && (op1_val.wrapping_add(op2_val)) < 0)
        || (op1_val < 0 && op2_val < 0 && (op1_val.wrapping_add(op2_val)) > 0)
    {
        set_flag(registers, "V", true);
    } else {
        set_flag(registers, "V", false);
    }
}

pub fn b(registers: &mut Vec<Register>, op1: &Operand) {
    let offset = op1.convert_reg_val(registers).unwrap().convert_64() as i64;
    let pc = get_register_value(registers, "PC").unwrap().convert_64() as i64;

    let new_pc = pc + offset - 4;

    set_register_value(registers, "PC", RegisterValue::Val64(new_pc as u64));
}

pub fn beq(registers: &mut Vec<Register>, op1: &Operand) {
    if get_flag(registers, "Z") {
        let offset = op1.convert_reg_val(registers).unwrap().convert_64() as i64;
        let pc = get_register_value(registers, "PC").unwrap().convert_64() as i64;

        let new_pc = pc + offset - 4;

        set_register_value(registers, "PC", RegisterValue::Val64(new_pc as u64));
    }
}

pub fn bne(registers: &mut Vec<Register>, op1: &Operand) {
    if !get_flag(registers, "Z") {
        let offset = op1.convert_reg_val(registers).unwrap().convert_64() as i64;
        let pc = get_register_value(registers, "PC").unwrap().convert_64() as i64;

        let new_pc = pc + offset - 4;

        set_register_value(registers, "PC", RegisterValue::Val64(new_pc as u64));
    }
}

pub fn bgt(registers: &mut Vec<Register>, op1: &Operand) {
    if !get_flag(registers, "Z") && get_flag(registers, "N") == get_flag(registers, "V") {
        let offset = op1.convert_reg_val(registers).unwrap().convert_64() as i64;
        let pc = get_register_value(registers, "PC").unwrap().convert_64() as i64;

        let new_pc = pc + offset - 4;

        set_register_value(registers, "PC", RegisterValue::Val64(new_pc as u64));
    }
}

pub fn blt(registers: &mut Vec<Register>, op1: &Operand) {
    if get_flag(registers, "N") != get_flag(registers, "V") {
        let offset = op1.convert_reg_val(registers).unwrap().convert_64() as i64;
        let pc = get_register_value(registers, "PC").unwrap().convert_64() as i64;

        let new_pc = pc + offset - 4;

        set_register_value(registers, "PC", RegisterValue::Val64(new_pc as u64));
    }
}

pub fn bge(registers: &mut Vec<Register>, op1: &Operand) {
    if get_flag(registers, "N") == get_flag(registers, "V") {
        let offset = op1.convert_reg_val(registers).unwrap().convert_64() as i64;
        let pc = get_register_value(registers, "PC").unwrap().convert_64() as i64;

        let new_pc = pc + offset - 4;

        set_register_value(registers, "PC", RegisterValue::Val64(new_pc as u64));
    }
}

pub fn svc(registers: &mut Vec<Register>, memory: &mut Vec<u8>) -> Result<(), String> {
    let n = get_register_value(registers, "X8").unwrap().convert_64();
    match n {
        63 => {
            sys_read(registers, memory);
        }
        64 => {
            sys_write(registers, memory);
        }
        93 => {
            sys_exit(registers);
        }
        _ => {
            fail(
                registers,
                &format!("Syscall #{} hasn't implemented yet.", n),
            );
            exit(1);
        }
    }

    Ok(())
}

pub fn adds(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &Operand) {
    let val1 = get_register_value(registers, op2).unwrap().convert_64() as u64;
    let val2 = op3.convert_reg_val(registers).unwrap().convert_64() as u64;
    let bits = if op1.chars().nth(0).unwrap() == 'W' {
        32
    } else {
        64
    };
    let mask: u64 = if bits == 32 {
        4294967295
    } else {
        18446744073709551615
    };
    let res;
    add(registers, op1, op2, op3.clone());

    res = get_register_value(registers, op1).unwrap().convert_64() as u64;

    set_flag(
        registers,
        "N",
        match res & (1 << (bits - 1)) {
            1 => true,
            _ => false,
        },
    );

    set_flag(
        registers,
        "Z",
        match res & mask {
            0 => true,
            _ => false,
        },
    );

    set_flag(registers, "C", res > mask);

    set_flag(
        registers,
        "V",
        (((val1 >> (bits - 1)) & 1) == ((val2 >> (bits - 1)) & 1))
            && (((val1 >> (bits - 1)) & 1) != (((res & mask) >> (bits - 1)) & 1)),
    );
}

pub fn subs(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &Operand) {
    let val1 = get_register_value(registers, op2).unwrap().convert_64() as i64;
    let val2 = op3.convert_reg_val(registers).unwrap().convert_64() as i64;
    let subtraction;
    sub(registers, op1, op2, op3.clone());

    subtraction = get_register_value(registers, op1).unwrap().convert_64() as i64;

    if subtraction < 0 {
        set_flag(registers, "N", true);
    } else {
        set_flag(registers, "N", false);
    }

    if subtraction == 0 {
        set_flag(registers, "Z", true);
    } else {
        set_flag(registers, "Z", false);
    }

    if val1 >= val2 {
        set_flag(registers, "C", true);
    } else {
        set_flag(registers, "C", false);
    }

    if (val1 > 0 && val2 > 0 && (val1.wrapping_add(val2)) < 0)
        || (val1 < 0 && val2 < 0 && (val1.wrapping_add(val2)) > 0)
    {
        set_flag(registers, "V", true);
    } else {
        set_flag(registers, "V", false);
    }
}

pub fn adr(registers: &mut Vec<Register>, op1: &str, op2: &Operand) {
    set_register_value(
        registers,
        op1,
        get_register_value(registers, "PC").unwrap() + op2.convert_reg_val(registers).unwrap(),
    );
}

pub fn movk(registers: &mut Vec<Register>, ins: &Instruction) {
    match ins.op1 {
        Some(Operand::OperandRegister(_)) => (),
        _ => {
            fail(registers, &format!("Invalid type in operand {}", 1));
            exit(1);
        }
    }

    match ins.op2 {
        Some(Operand::OperandNumber(_)) => (),
        _ => {
            fail(registers, &format!("Invalid type in operand {}", 2));
            exit(1);
        }
    }

    let bit_start: u8 = match ins.barrelshifter.as_ref() {
        Some(n) => {
            match n.barrelshiftertype {
                BarrelShifterType::LSL => (),
                _ => {
                    fail(registers, "Barrel shifting else LSL not allowed for MOVK.");
                    exit(1);
                }
            }
            n.value
                .unwrap_or_else(|| {
                    fail(
                        registers,
                        "Providing a value for the barrel shifter LSL is required.",
                    );
                    exit(1);
                })
                .convert_64()
                .try_into()
                .unwrap_or_else(|_| {
                    fail(
                        registers,
                        "LSL can't take a value which isn't 8-bits for MOVK.",
                    );
                    exit(1);
                })
        }
        None => 0,
    };

    let reg_name = ins.op1.as_ref().unwrap().get_reg_value().unwrap();
    let value = match ins.op2 {
        Some(Operand::OperandNumber(n)) => n,
        _ => unreachable!(),
    }
    .convert_64();

    if reg_name.chars().nth(0).unwrap() == 'W' {
        let mask: u32 = 0xffff << bit_start;
        set_register_value(
            registers,
            reg_name,
            RegisterValue::Val32(
                ((ins
                    .op1
                    .as_ref()
                    .unwrap()
                    .convert_reg_val(registers)
                    .unwrap()
                    .convert_32())
                    & !mask)
                    | (((value as u32) << bit_start) & mask),
            ),
        );
    } else {
        let mask: u64 = 0xffff << bit_start;
        set_register_value(
            registers,
            reg_name,
            RegisterValue::Val64(
                ((ins
                    .op1
                    .as_ref()
                    .unwrap()
                    .convert_reg_val(registers)
                    .unwrap()
                    .convert_64())
                    & !mask)
                    | ((value << bit_start) & mask),
            ),
        );
    }
}

pub fn adrp(registers: &mut Vec<Register>, op1: &str, op2: &Operand) {
    set_register_value(
        registers,
        op1,
        RegisterValue::Val64(
            (get_register_value(registers, "PC").unwrap().convert_64() & 0xfffffffffffff000)
                + (op2.convert_reg_val(registers).unwrap().convert_64() << 12),
        ),
    );
}

pub fn orr(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &Operand) {
    set_register_value(
        registers,
        op1,
        get_register_value(registers, op2).unwrap() | op3.convert_reg_val(registers).unwrap(),
    );
}

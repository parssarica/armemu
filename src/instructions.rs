use crate::instruction_parser::*;
use crate::registers::*;

pub enum OperandType {
    Register,
    Immediate,
    MemoryAddress,
    RegImm,
    RegMem,
    Triple,
}

pub enum Instructions {
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
        op2: MemoryAddress,
    },
    Str {
        op1: String,
        op2: MemoryAddress,
    },
}

pub fn convert_ins(ins: &Instruction) -> Result<Instructions, String> {
    match ins.name.to_lowercase().as_str() {
        "mov" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            None,
        )?,
        "add" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
        )?,
        "sub" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
        )?,
        "mul" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
        )?,
        "and" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
        )?,
        "ldr" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::MemoryAddress),
            None,
            None,
        )?,
        "str" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::MemoryAddress),
            None,
            None,
        )?,
        _ => return Err(format!("Unknown instruction: {}", ins.name.as_str())),
    }

    Ok(match ins.name.to_lowercase().as_str() {
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
            op2: match ins.op2.as_ref().unwrap() {
                Operand::OperandAddress(n) => n.clone(),
                _ => unreachable!(),
            },
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
        _ => unreachable!(),
    })
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
            Operand::OperandRegister(_) => match op1_type.unwrap() {
                OperandType::Immediate | OperandType::MemoryAddress => {
                    return Err("Invalid type in operand 1".to_string())
                }
                _ => (),
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
            Operand::OperandRegister(_) => match op2_type.unwrap() {
                OperandType::Immediate | OperandType::MemoryAddress => {
                    return Err("Invalid type in operand 2".to_string())
                }
                _ => (),
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
            Operand::OperandRegister(_) => match op3_type.unwrap() {
                OperandType::Immediate | OperandType::MemoryAddress => {
                    return Err("Invalid type in operand 3".to_string())
                }
                _ => (),
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
            Operand::OperandRegister(_) => match op4_type.unwrap() {
                OperandType::Immediate | OperandType::MemoryAddress => {
                    return Err("Invalid type in operand 4".to_string())
                }
                _ => (),
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
    op2: &MemoryAddress,
    memory: &Vec<u8>,
) -> Result<(), String> {
    op2.change_reg_preindex(registers);

    match op2.addr_type {
        MemoryAddressType::SetRegister => {
            set_register_value(
                registers,
                op1,
                op2.second_val.as_ref().unwrap().get_val(registers).unwrap(),
            );
            return Ok(());
        }
        _ => (),
    }

    let addr = op2.get_addr(registers).convert_64();
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

    op2.change_reg_postindex(registers);

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

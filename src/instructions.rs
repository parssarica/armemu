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

pub fn mov(
    registers: &mut Vec<Register>,
    instruction: &Instruction,
    output: &mut Result<(), String>,
) {
    match operand_check(
        instruction,
        Some(OperandType::Register),
        Some(OperandType::RegImm),
        None,
        None,
    ) {
        Err(n) => {
            *output = Err(n);
            return;
        }
        Ok(_) => (),
    }

    set_register_value(
        registers,
        instruction.op1.as_ref().unwrap().get_reg_value().unwrap(),
        match &instruction.op2 {
            Some(Operand::OperandRegister(n)) => match get_register(registers, &n) {
                Some(n2) => n2.value,
                _ => unreachable!(),
            },
            Some(Operand::OperandNumber(n)) => *n,
            _ => unreachable!(),
        },
    );

    *output = Ok(());
}

pub fn add(
    registers: &mut Vec<Register>,
    instruction: &Instruction,
    output: &mut Result<(), String>,
) {
    match operand_check(
        instruction,
        Some(OperandType::Register),
        Some(OperandType::Register),
        Some(OperandType::RegImm),
        None,
    ) {
        Err(n) => {
            *output = Err(n);
            return;
        }
        Ok(_) => (),
    }

    let register_name = &(get_register(
        registers,
        instruction.op1.as_ref().unwrap().get_reg_value().unwrap(),
    )
    .unwrap()
    .name
    .clone());

    set_register_value(
        registers,
        &register_name,
        (get_register(
            registers,
            instruction.op2.as_ref().unwrap().get_reg_value().unwrap(),
        )
        .unwrap()
        .value
            + match instruction.op3.as_ref().unwrap() {
                Operand::OperandRegister(n) => get_register(registers, &n).unwrap().value,
                Operand::OperandNumber(n) => *n,
                _ => unreachable!(),
            })
        .convert_reg(&register_name),
    );

    *output = Ok(());
}

pub fn sub(
    registers: &mut Vec<Register>,
    instruction: &Instruction,
    output: &mut Result<(), String>,
) {
    match operand_check(
        instruction,
        Some(OperandType::Register),
        Some(OperandType::Register),
        Some(OperandType::RegImm),
        None,
    ) {
        Err(n) => {
            *output = Err(n);
            return;
        }
        Ok(_) => (),
    }

    let register_name = &(get_register(
        registers,
        instruction.op1.as_ref().unwrap().get_reg_value().unwrap(),
    )
    .unwrap()
    .name
    .clone());

    set_register_value(
        registers,
        &register_name,
        (get_register(
            registers,
            instruction.op2.as_ref().unwrap().get_reg_value().unwrap(),
        )
        .unwrap()
        .value
            - match instruction.op3.as_ref().unwrap() {
                Operand::OperandRegister(n) => get_register(registers, &n).unwrap().value,
                Operand::OperandNumber(n) => *n,
                _ => unreachable!(),
            })
        .convert_reg(&register_name),
    );

    *output = Ok(());
}

pub fn mul(
    registers: &mut Vec<Register>,
    instruction: &Instruction,
    output: &mut Result<(), String>,
) {
    match operand_check(
        instruction,
        Some(OperandType::Register),
        Some(OperandType::Register),
        Some(OperandType::RegImm),
        None,
    ) {
        Err(n) => {
            *output = Err(n);
            return;
        }
        Ok(_) => (),
    }

    let register_name = &(get_register(
        registers,
        instruction.op1.as_ref().unwrap().get_reg_value().unwrap(),
    )
    .unwrap()
    .name
    .clone());

    set_register_value(
        registers,
        &register_name,
        (get_register(
            registers,
            instruction.op2.as_ref().unwrap().get_reg_value().unwrap(),
        )
        .unwrap()
        .value
            * match instruction.op3.as_ref().unwrap() {
                Operand::OperandRegister(n) => get_register(registers, &n).unwrap().value,
                Operand::OperandNumber(n) => *n,
                _ => unreachable!(),
            })
        .convert_reg(&register_name),
    );

    *output = Ok(());
}

pub fn and(
    registers: &mut Vec<Register>,
    instruction: &Instruction,
    output: &mut Result<(), String>,
) {
    match operand_check(
        instruction,
        Some(OperandType::Register),
        Some(OperandType::Register),
        Some(OperandType::RegImm),
        None,
    ) {
        Err(n) => {
            *output = Err(n);
            return;
        }
        Ok(_) => (),
    }

    let register_name = &(get_register(
        registers,
        instruction.op1.as_ref().unwrap().get_reg_value().unwrap(),
    )
    .unwrap()
    .name
    .clone());

    set_register_value(
        registers,
        &register_name,
        (get_register(
            registers,
            instruction.op2.as_ref().unwrap().get_reg_value().unwrap(),
        )
        .unwrap()
        .value
            & match instruction.op3.as_ref().unwrap() {
                Operand::OperandRegister(n) => get_register(registers, &n).unwrap().value,
                Operand::OperandNumber(n) => *n,
                _ => unreachable!(),
            })
        .convert_reg(&register_name),
    );

    *output = Ok(());
}

pub fn ldr(
    registers: &mut Vec<Register>,
    instruction: &Instruction,
    output: &mut Result<(), String>,
    memory: &Vec<u8>,
) {
    match operand_check(
        instruction,
        Some(OperandType::Register),
        Some(OperandType::MemoryAddress),
        None,
        None,
    ) {
        Err(n) => {
            *output = Err(n);
            return;
        }
        Ok(_) => (),
    }

    let op1 = match instruction.op1.as_ref().unwrap() {
        Operand::OperandRegister(n) => n,
        _ => unreachable!(),
    };

    let op2 = match instruction.op2.as_ref().unwrap() {
        Operand::OperandAddress(n) => n,
        _ => unreachable!(),
    };

    match op2.addr_type {
        MemoryAddressType::SetRegister => {
            set_register_value(registers, &op1, op2.second_val.unwrap());
            return;
        }
        _ => (),
    }

    let reg_name = instruction.op1.as_ref().unwrap().get_reg_value().unwrap();
    let addr = match (match instruction.op2.as_ref().unwrap() {
        Operand::OperandAddress(n) => n,
        _ => unreachable!(),
    })
    .get_addr(registers)
    {
        RegisterValue::Val32(n) => n as u64,
        RegisterValue::Val64(n) => n,
    };
    let mut bytes = Vec::new();
    let is_32_bit = match reg_name.chars().nth(0).unwrap() {
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
                *output = Err(String::from("Invalid memory address"));
                return;
            }
        });
    }

    bytes.reverse();

    if is_32_bit {
        set_register_value(
            registers,
            reg_name,
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
                    *output = Err("Problem when parsing data from memory".to_string());
                    return;
                }
            },
        );
    } else {
        set_register_value(
            registers,
            reg_name,
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
                    *output = Err("Problem when parsing data from memory".to_string());
                    return;
                }
            },
        );
    }

    (match instruction.op2.as_ref().unwrap() {
        Operand::OperandAddress(n) => n,
        _ => unreachable!(),
    })
    .change_reg(registers);
}

pub fn str(
    registers: &mut Vec<Register>,
    instruction: &Instruction,
    output: &mut Result<(), String>,
    memory: &mut Vec<u8>,
) {
    match operand_check(
        instruction,
        Some(OperandType::Register),
        Some(OperandType::MemoryAddress),
        None,
        None,
    ) {
        Err(n) => {
            *output = Err(n);
            return;
        }
        Ok(_) => (),
    }

    let reg_name = instruction.op1.as_ref().unwrap().get_reg_value().unwrap();
    let addr = match (match instruction.op2.as_ref().unwrap() {
        Operand::OperandAddress(n) => n,
        _ => unreachable!(),
    })
    .get_addr(registers)
    {
        RegisterValue::Val32(n) => n as u64,
        RegisterValue::Val64(n) => n,
    };

    let bytes: Vec<String> =
        group_couple(format!("{:016X}", get_register_value(registers, reg_name).unwrap()).as_str())
            .iter()
            .rev()
            .cloned()
            .collect();
    let mut j = 0;

    for _ in &bytes {
        memory[(addr + j) as usize] =
            u8::from_str_radix(bytes.get(j as usize).unwrap(), 16).unwrap();
        j += 1;
    }

    (match instruction.op2.as_ref().unwrap() {
        Operand::OperandAddress(n) => n,
        _ => unreachable!(),
    })
    .change_reg(registers);
}

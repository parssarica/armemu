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

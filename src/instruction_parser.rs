use crate::registers::*;

#[derive(Clone)]
pub enum BarrelShifterType {
    LSL,
    LSR,
    ASR,
    ROR,
    RRX,
}

#[derive(Clone)]
pub struct BarrelShifter {
    pub barrelshiftertype: BarrelShifterType,
    pub value: Option<RegisterValue>,
}

pub enum Operand {
    OperandRegister(String),
    OperandNumber(RegisterValue),
}

pub struct Instruction {
    pub name: String,
    pub op1: Option<Operand>,
    pub op2: Option<Operand>,
    pub op3: Option<Operand>,
    pub op4: Option<Operand>,
    pub barrelshifter: Option<BarrelShifter>,
    pub operand_count: u8,
}

impl Operand {
    pub fn get_reg_value(&self) -> Option<&str> {
        match self {
            Self::OperandRegister(n) => Some(&n),
            Self::OperandNumber(_) => None,
        }
    }

    pub fn get_num_value(&self) -> Option<RegisterValue> {
        match self {
            Self::OperandNumber(n) => Some(*n),
            Self::OperandRegister(_) => None,
        }
    }
}

pub fn parse_instruction(line: &str, registers: &Vec<Register>) -> Option<Instruction> {
    let mut i = 0;
    let mut trimmed_parts: Vec<&str>;
    let mut operand: Option<Operand>;
    let mut parts: Vec<&str>;
    let mut op1: Option<Operand> = None;
    let mut op2: Option<Operand> = None;
    let mut op3: Option<Operand> = None;
    let mut op4: Option<Operand> = None;
    let mut barrelshifter: Option<BarrelShifter> = None;
    let mut operand_count = 0;
    let instruction_name: Option<String>;

    if let Some((beginning, rest)) = line.split_once(' ') {
        instruction_name = Some(beginning.to_string());
        parts = rest.split(',').collect();
    } else {
        return None;
    }

    if parts.len() < 1 {
        return None;
    }

    for part in &mut parts {
        if i > 5 {
            return None;
        }

        trimmed_parts = part
            .trim()
            .trim_matches(|c: char| c == ',')
            .trim()
            .split(" ")
            .collect();

        if trimmed_parts.len() < 1 {
            return None;
        } else if trimmed_parts.len() == 1 {
            operand = match get_register(registers, trimmed_parts[0]) {
                Some(n) => Some(Operand::OperandRegister(n.name.clone())),
                None => match trimmed_parts[0]
                    .trim_matches(|c: char| c == '#')
                    .parse::<u64>()
                {
                    Ok(n) => Some(Operand::OperandNumber(RegisterValue::Val64(n))),
                    Err(_) => None,
                },
            };
            match i {
                0 => op1 = operand,
                1 => op2 = operand,
                2 => op3 = operand,
                3 => op4 = operand,
                _ => return None,
            }

            operand_count += 1;
        } else if trimmed_parts.len() == 2 {
            barrelshifter = Some(BarrelShifter {
                barrelshiftertype: match trimmed_parts[0] {
                    "LSL" => BarrelShifterType::LSL,
                    "LSR" => BarrelShifterType::LSR,
                    "ASR" => BarrelShifterType::ASR,
                    "ROR" => BarrelShifterType::ROR,
                    "RRX" => BarrelShifterType::RRX,
                    _ => return None,
                },
                value: match trimmed_parts[1]
                    .trim_matches(|c: char| c == '#')
                    .parse::<u64>()
                    .ok()
                {
                    Some(n) => Some(RegisterValue::Val64(n)),
                    None => match get_register(registers, trimmed_parts[1]) {
                        Some(n) => Some(n.value),
                        None => None,
                    },
                },
            });
            break;
        } else {
            return None;
        }

        i += 1;
    }

    Some(Instruction {
        name: instruction_name?,
        op1: op1,
        op2: op2,
        op3: op3,
        op4: op4,
        barrelshifter: barrelshifter,
        operand_count: operand_count,
    })
}

pub fn parse_file(registers: &Vec<Register>, file: &str) -> Option<Vec<Instruction>> {
    let mut ins: Instruction;
    let mut code = Vec::new();

    for line in file.lines() {
        ins = parse_instruction(line, registers)?;
        code.push(ins);
    }

    Some(code)
}

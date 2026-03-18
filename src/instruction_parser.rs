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

pub enum Operand<'a> {
    OperandRegister(&'a Register),
    OperandNumber(RegisterValue),
}

pub struct Instruction<'a> {
    pub name: String,
    pub op1: Option<Operand<'a>>,
    pub op2: Option<Operand<'a>>,
    pub op3: Option<Operand<'a>>,
    pub op4: Option<Operand<'a>>,
    pub barrelshifter: Option<BarrelShifter>,
}

pub fn parse_instruction<'a>(
    line: &'a str,
    registers: &'a Vec<Register>,
) -> Option<Instruction<'a>> {
    let mut i = 0;
    let mut trimmed_parts: Vec<&str>;
    let mut operand: Option<Operand>;
    let mut parts: Vec<&str>;
    let mut op1: Option<Operand> = None;
    let mut op2: Option<Operand> = None;
    let mut op3: Option<Operand> = None;
    let mut op4: Option<Operand> = None;
    let mut barrelshifter: Option<BarrelShifter> = None;
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
                Some(n) => Some(Operand::OperandRegister(n)),
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
    })
}

pub fn parse_file<'a>(registers: &'a Vec<Register>, file: &'a str) -> Option<Vec<Instruction<'a>>> {
    let mut ins: Instruction;
    let mut code = Vec::new();

    for line in file.lines() {
        ins = parse_instruction(line, registers)?;
        code.push(ins);
    }

    Some(code)
}

use crate::registers::*;
use std::fmt;

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

#[derive(Clone)]
pub enum MemoryAddressType {
    Preindexed,
    Postindexed,
    Normal,
    SetRegister,
    RegisterOffset,
}

#[derive(Clone)]
pub struct MemoryAddress {
    pub base_address: String,
    pub second_val: Option<MemoryAddressVal>,
    pub barrelshifter: Option<BarrelShifter>,
    pub postindexval: Option<RegisterValue>,
    pub addr_type: MemoryAddressType,
}

#[derive(Clone)]
pub enum MemoryAddressVal {
    ValRegister(String),
    ValNumber(RegisterValue),
}

#[derive(Clone)]
pub enum Operand {
    OperandRegister(String),
    OperandNumber(RegisterValue),
    OperandAddress(MemoryAddress),
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

impl MemoryAddressVal {
    pub fn get_val(&self, registers: &Vec<Register>) -> Option<RegisterValue> {
        match self {
            Self::ValRegister(n) => get_register_value(registers, n),
            Self::ValNumber(n) => Some(*n),
        }
    }
}

impl BarrelShifter {
    pub fn parse_val(
        &self,
        registers: &mut Vec<Register>,
        regval: RegisterValue,
    ) -> Option<RegisterValue> {
        if self.value.is_none() {
            return None;
        }

        match self.barrelshiftertype {
            BarrelShifterType::LSL => {
                let val = regval;
                let n = self.value.unwrap();
                set_flag(
                    registers,
                    "C",
                    match (val >> RegisterValue::Val64(32 - n.convert_64())).convert_64() & 1 {
                        0 => false,
                        1 => true,
                        _ => unreachable!(),
                    },
                );
                Some(val << n)
            }
            BarrelShifterType::LSR => {
                let val = regval;
                let n = self.value.unwrap();
                set_flag(
                    registers,
                    "C",
                    match (val.convert_64() >> (n.convert_64() - 1)) & 1 {
                        0 => false,
                        1 => true,
                        _ => unreachable!(),
                    },
                );
                Some(val >> n)
            }
            BarrelShifterType::ASR => {
                let val = (regval).convert_32() as i32;
                let n = self.value.unwrap();
                set_flag(
                    registers,
                    "C",
                    match (val >> (n.convert_64() - 1)) & 1 {
                        0 => false,
                        1 => true,
                        _ => unreachable!(),
                    },
                );
                Some(RegisterValue::Val32(
                    (val >> self.value.unwrap().convert_32()) as u32,
                ))
            }
            BarrelShifterType::ROR => {
                let val = regval;
                let n = self.value.unwrap();

                set_flag(
                    registers,
                    "C",
                    match ((val >> RegisterValue::Val64(n.convert_64() - 1))
                        & RegisterValue::Val64(1))
                    .convert_64()
                    {
                        0 => false,
                        1 => true,
                        _ => unreachable!(),
                    },
                );
                Some((val >> n) | (val << (RegisterValue::Val64(32) - n)))
            }
            BarrelShifterType::RRX => {
                let val = regval;
                set_flag(
                    registers,
                    "C",
                    match val.convert_64() & 1 {
                        0 => false,
                        1 => true,
                        _ => unreachable!(),
                    },
                );
                Some(RegisterValue::Val64(
                    (val.convert_64() >> 1)
                        | ((match get_flag(registers, "C") {
                            false => 0,
                            true => 1,
                        }) << 31),
                ))
            }
        }
    }
}

impl MemoryAddress {
    pub fn get_addr(&self, registers: &Vec<Register>) -> RegisterValue {
        get_register_value(registers, &self.base_address).unwrap()
            + match self.addr_type {
                MemoryAddressType::Preindexed | MemoryAddressType::RegisterOffset => self
                    .second_val
                    .as_ref()
                    .unwrap()
                    .get_val(registers)
                    .unwrap(),
                _ => RegisterValue::Val64(0),
            }
    }

    pub fn change_reg_postindex(&self, registers: &mut Vec<Register>) {
        match self.addr_type {
            MemoryAddressType::Postindexed => set_register_value(
                registers,
                &self.base_address,
                get_register_value(registers, &self.base_address).unwrap()
                    + self.postindexval.unwrap(),
            ),
            _ => (),
        }
    }

    pub fn change_reg_preindex(&self, registers: &mut Vec<Register>) {
        match self.addr_type {
            MemoryAddressType::Preindexed => set_register_value(
                registers,
                &self.base_address,
                get_register_value(registers, &self.base_address).unwrap()
                    + self
                        .second_val
                        .as_ref()
                        .unwrap()
                        .get_val(registers)
                        .unwrap(),
            ),
            _ => (),
        }
    }
}

impl Operand {
    pub fn get_reg_value(&self) -> Option<&str> {
        match self {
            Self::OperandRegister(n) => Some(&n),
            _ => None,
        }
    }

    pub fn get_num_value(&self) -> Option<RegisterValue> {
        match self {
            Self::OperandNumber(n) => Some(*n),
            _ => None,
        }
    }

    pub fn convert_reg_val(&self, registers: &Vec<Register>) -> Option<RegisterValue> {
        match self {
            Self::OperandRegister(n) => Some(get_register_value(registers, n)?),
            Self::OperandNumber(n) => Some(*n),
            _ => None,
        }
    }
}

impl fmt::Display for MemoryAddressVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ValRegister(r) => write!(f, "{}", r),
            Self::ValNumber(n) => write!(f, "{}", n),
        }
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OperandRegister(s) => write!(f, "{}", s),
            Self::OperandNumber(s) => write!(f, "#{}", s),
            Self::OperandAddress(s) => {
                if s.barrelshifter.is_some() {
                    match s.addr_type {
                        MemoryAddressType::Preindexed => {
                            return write!(
                                f,
                                "[{}, {}, {}]!",
                                s.base_address,
                                s.second_val.as_ref().unwrap(),
                                s.barrelshifter.as_ref().unwrap()
                            )
                        }
                        MemoryAddressType::Postindexed => {
                            return write!(
                                f,
                                "[{}, {}], #{}",
                                s.base_address,
                                s.barrelshifter.as_ref().unwrap(),
                                s.postindexval.unwrap()
                            )
                        }
                        MemoryAddressType::Normal => {
                            return write!(
                                f,
                                "[{}, {}]",
                                s.base_address,
                                s.barrelshifter.as_ref().unwrap()
                            )
                        }
                        MemoryAddressType::SetRegister => {
                            return write!(f, "={}", s.second_val.as_ref().unwrap());
                        }
                        MemoryAddressType::RegisterOffset => {
                            return write!(
                                f,
                                "[{}, {}]",
                                s.base_address,
                                s.second_val.as_ref().unwrap()
                            );
                        }
                    }
                } else {
                    match s.addr_type {
                        MemoryAddressType::Preindexed => {
                            return write!(
                                f,
                                "[{}, {}]!",
                                s.base_address,
                                s.second_val.as_ref().unwrap()
                            )
                        }
                        MemoryAddressType::Postindexed => {
                            return write!(f, "[{}], #{}", s.base_address, s.postindexval.unwrap())
                        }
                        MemoryAddressType::Normal => return write!(f, "[{}]", s.base_address),
                        MemoryAddressType::SetRegister => {
                            return write!(f, "={}", s.second_val.as_ref().unwrap());
                        }
                        MemoryAddressType::RegisterOffset => {
                            return write!(
                                f,
                                "[{}, {}]",
                                s.base_address,
                                s.second_val.as_ref().unwrap()
                            );
                        }
                    }
                }
            }
        }
    }
}

impl fmt::Display for BarrelShifter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let prefix = match self.barrelshiftertype {
            BarrelShifterType::LSL => "LSL",
            BarrelShifterType::LSR => "LSR",
            BarrelShifterType::ASR => "ASR",
            BarrelShifterType::ROR => "ROR",
            BarrelShifterType::RRX => "RRX",
        };

        match self.value {
            Some(n) => write!(f, "{} #{}", prefix, n),
            None => write!(f, "{}", prefix),
        }
    }
}

pub fn is_label(line: &str) -> bool {
    if line.len() <= 1 {
        return false;
    }

    if !line.ends_with(":") {
        return false;
    }

    if line[..(line.len() - 1)]
        .chars()
        .any(|i: char| i < '0' || (i > '9' && i < 'A') || (i > 'Z' && i < 'a') || i > 'z')
    {
        return false;
    }

    true
}

pub fn parse_labels(file: &str) -> Vec<(&str, u64)> {
    let mut labels = Vec::new();
    let mut i: u64 = 0;

    for line in file.lines() {
        if is_label(line.trim()) {
            labels.push((&line.trim()[..(line.len() - 1)], i));
        }

        i += 1;
    }

    labels
}

pub fn parse_memory_address(registers: &Vec<Register>, line: &str) -> Option<Operand> {
    if !line.contains("[") || !line.contains("]") {
        return None;
    }

    let registername: String;
    let mut postindexval: Option<RegisterValue> = None;
    let mut addr_type: MemoryAddressType = MemoryAddressType::Normal;
    let mut barrelshifter: Option<BarrelShifter> = None;
    let mut second_val: Option<MemoryAddressVal> = None;

    if line.contains("!") {
        addr_type = MemoryAddressType::Preindexed;
    }

    let address = line
        .replace("!", "")
        .replace("[", "")
        .replace("]", "")
        .replace(" ", "");

    let first_part = address.split(',').next()?;

    let second_part = address.split(',').nth(1);

    let third_part = match second_part {
        Some(_) => address.split(',').nth(2),
        None => None,
    };

    let fourth_part = match third_part {
        Some(_) => address.split(',').nth(3),
        None => None,
    };

    let value: RegisterValue;

    registername = first_part.trim().to_string();

    match second_part {
        Some(n) => {
            if n.trim().starts_with("#") {
                match registername.chars().nth(0).unwrap() {
                    'W' => value = RegisterValue::Val32(n.trim()[1..].parse::<u32>().ok()?),
                    _ => value = RegisterValue::Val64(n.trim()[1..].parse::<u64>().ok()?),
                }
                if line.find("]")? != line.replace("!", "").len() - 1 {
                    postindexval = Some(value);
                    addr_type = MemoryAddressType::Postindexed;
                } else {
                    second_val = Some(MemoryAddressVal::ValNumber(value));
                }
            } else {
                value = get_register_value(registers, n.trim())?;

                if line.find("]")? != line.len() - 1 {
                    postindexval = Some(value);
                } else {
                    second_val = Some(MemoryAddressVal::ValRegister(n.trim().to_string()));
                }
            }
        }
        None => {
            return Some(Operand::OperandAddress(MemoryAddress {
                base_address: registername,
                second_val: None,
                postindexval: None,
                barrelshifter: None,
                addr_type: addr_type,
            }))
        }
    }

    if matches!(addr_type, MemoryAddressType::Normal) && second_val.is_some() {
        addr_type = MemoryAddressType::RegisterOffset;
    }

    if line.chars().nth(0).unwrap() == '=' {
        return Some(Operand::OperandAddress(MemoryAddress {
            base_address: registername,
            second_val: second_val,
            barrelshifter: None,
            postindexval: None,
            addr_type: MemoryAddressType::SetRegister,
        }));
    }

    match third_part {
        Some(n) => {
            if n.trim().starts_with("#") {
                postindexval = match registername.chars().nth(0).unwrap() {
                    'W' => Some(RegisterValue::Val32(n.trim()[1..].parse::<u32>().ok()?)),
                    _ => Some(RegisterValue::Val64(n.trim()[1..].parse::<u64>().ok()?)),
                }
            } else if n.trim().starts_with("LSL") {
                barrelshifter = Some(BarrelShifter {
                    barrelshiftertype: BarrelShifterType::LSL,
                    value: match registername.chars().nth(0).unwrap() {
                        'W' => Some(RegisterValue::Val32(
                            n.replace("LSL", "").trim().parse::<u32>().ok()?,
                        )),
                        _ => Some(RegisterValue::Val64(
                            n.replace("LSL", "").trim().parse::<u64>().ok()?,
                        )),
                    },
                })
            } else if n.trim().starts_with("LSR") {
                barrelshifter = Some(BarrelShifter {
                    barrelshiftertype: BarrelShifterType::LSR,
                    value: match registername.chars().nth(0).unwrap() {
                        'W' => Some(RegisterValue::Val32(
                            n.replace("LSR", "").trim().parse::<u32>().ok()?,
                        )),
                        _ => Some(RegisterValue::Val64(
                            n.replace("LSR", "").trim().parse::<u64>().ok()?,
                        )),
                    },
                })
            } else if n.trim().starts_with("ASR") {
                barrelshifter = Some(BarrelShifter {
                    barrelshiftertype: BarrelShifterType::ASR,
                    value: match registername.chars().nth(0).unwrap() {
                        'W' => Some(RegisterValue::Val32(
                            n.replace("ASR", "").trim().parse::<u32>().ok()?,
                        )),
                        _ => Some(RegisterValue::Val64(
                            n.replace("ASR", "").trim().parse::<u64>().ok()?,
                        )),
                    },
                })
            } else if n.trim().starts_with("ROR") {
                barrelshifter = Some(BarrelShifter {
                    barrelshiftertype: BarrelShifterType::ROR,
                    value: match registername.chars().nth(0).unwrap() {
                        'W' => Some(RegisterValue::Val32(
                            n.replace("ROR", "").trim().parse::<u32>().ok()?,
                        )),
                        _ => Some(RegisterValue::Val64(
                            n.replace("ROR", "").trim().parse::<u64>().ok()?,
                        )),
                    },
                })
            } else if n.trim().starts_with("RRX") {
                barrelshifter = Some(BarrelShifter {
                    barrelshiftertype: BarrelShifterType::RRX,
                    value: match registername.chars().nth(0).unwrap() {
                        'W' => Some(RegisterValue::Val32(
                            n.replace("RRX", "").trim().parse::<u32>().ok()?,
                        )),
                        _ => Some(RegisterValue::Val64(
                            n.replace("RRX", "").trim().parse::<u64>().ok()?,
                        )),
                    },
                })
            }
        }
        None => {
            return Some(Operand::OperandAddress(MemoryAddress {
                base_address: registername,
                second_val: second_val,
                barrelshifter: barrelshifter,
                addr_type: addr_type,
                postindexval: postindexval,
            }));
        }
    }

    match fourth_part {
        Some(n) => {
            if n.trim().starts_with("#") {
                postindexval = match registername.chars().nth(0).unwrap() {
                    'W' => Some(RegisterValue::Val32(n.trim()[1..].parse::<u32>().ok()?)),
                    _ => Some(RegisterValue::Val64(n.trim()[1..].parse::<u64>().ok()?)),
                }
            }
        }
        None => (),
    }

    Some(Operand::OperandAddress(MemoryAddress {
        base_address: registername,
        second_val: None,
        barrelshifter: barrelshifter,
        postindexval: postindexval,
        addr_type: addr_type,
    }))
}

pub fn magic_split(line: &str) -> Option<Vec<String>> {
    let mut parts: Vec<String> = Vec::new();
    let mut is_address = false;
    let mut last_idx;
    let rest_parts: Vec<&str>;

    if let Some((beginning, rest)) = line.split_once(' ') {
        parts.push(beginning.to_string());
        rest_parts = rest.split(',').collect();
    } else {
        return None;
    }

    for part in rest_parts {
        let trimmed = part.trim();
        if part.contains("[") && !part.contains("]") {
            is_address = true;
        }

        if !is_address && parts.len() > 1 && parts[parts.len() - 1].contains("]") {
            last_idx = parts.len() - 1;
            parts[last_idx].push_str(&format!(", {}", part.trim()));
            continue;
        }

        if !is_address {
            parts.push(trimmed.to_string());
        } else {
            if parts.len() > 1 && !part.contains("[") {
                last_idx = parts.len() - 1;
                parts[last_idx].push_str(&format!(", {}", part.trim()));
            } else {
                parts.push(String::from(part.trim()));
            }
        }

        if is_address && part.contains("]") {
            is_address = false;
        }
    }

    if parts.len() < 1 {
        return None;
    }

    Some(parts)
}

pub fn parse_instruction(
    line: &str,
    registers: &Vec<Register>,
    labels: &Vec<(&str, u64)>,
) -> Option<Instruction> {
    let mut i = 0;
    let mut trimmed_parts: Vec<&str>;
    let mut operand: Option<Operand>;
    let mut parts: Vec<String>;
    let mut op1: Option<Operand> = None;
    let mut op2: Option<Operand> = None;
    let mut op3: Option<Operand> = None;
    let mut op4: Option<Operand> = None;
    let mut barrelshifter: Option<BarrelShifter> = None;
    let mut operand_count = 0;
    let instruction_name: Option<String>;

    parts = magic_split(line)?;

    instruction_name = Some(parts[0].clone());

    for part in &mut parts[1..] {
        if i > 5 {
            return None;
        }

        trimmed_parts = part.split(' ').collect();
        if part.len() < 1 {
            return None;
        } else if part.matches(' ').count() == 0 {
            operand = match get_register(registers, part) {
                Some(n) => Some(Operand::OperandRegister(n.name.clone())),
                None => match trimmed_parts[0]
                    .trim_matches(|c: char| c == '#')
                    .parse::<u64>()
                {
                    Ok(n) => Some(Operand::OperandNumber(RegisterValue::Val64(n))),
                    Err(_) => match trimmed_parts[0]
                        .trim_matches(|c: char| c == '#')
                        .strip_prefix("0x")
                    {
                        Some(k) => Some(Operand::OperandNumber(RegisterValue::Val64(
                            u64::from_str_radix(k, 16).ok()?,
                        ))),
                        None => match parse_memory_address(registers, part) {
                            Some(k) => Some(k),
                            None => match labels.iter().find(|&&x| x.0 == part) {
                                Some(k) => Some(Operand::OperandNumber(RegisterValue::Val64(k.1))),
                                None => None,
                            },
                        },
                    },
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
        } else if part.contains("[") {
            operand = Some(parse_memory_address(registers, part)?);

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

pub fn parse_file(
    registers: &Vec<Register>,
    file: &str,
    labels: &Vec<(&str, u64)>,
) -> Option<Vec<Instruction>> {
    let mut ins: Instruction;
    let mut code = Vec::new();

    for line in file.lines() {
        if is_label(line) {
            continue;
        }

        ins = parse_instruction(line, registers, labels)?;
        code.push(ins);
    }

    Some(code)
}

pub fn get_bit(val: RegisterValue, bit: u8) -> bool {
    match match val {
        RegisterValue::Val64(n) => (n >> bit) & 1,
        RegisterValue::Val32(n) => ((n >> bit) & 1) as u64,
    } {
        0 => false,
        1 => true,
        _ => unreachable!(),
    }
}

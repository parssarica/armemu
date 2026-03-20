use std::ops::{Add, BitAnd, Mul, Sub};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RegisterValue {
    Val32(u32),
    Val64(u64),
}

impl Add for RegisterValue {
    type Output = RegisterValue;

    fn add(self, other: RegisterValue) -> RegisterValue {
        let val1 = match self {
            Self::Val32(n) => n as u64,
            Self::Val64(n) => n,
        };

        let val2 = match other {
            Self::Val32(n) => n as u64,
            Self::Val64(n) => n,
        };

        RegisterValue::Val64(val1 + val2)
    }
}

impl Sub for RegisterValue {
    type Output = RegisterValue;

    fn sub(self, other: RegisterValue) -> RegisterValue {
        let val1 = match self {
            Self::Val32(n) => n as u64,
            Self::Val64(n) => n,
        };

        let val2 = match other {
            Self::Val32(n) => n as u64,
            Self::Val64(n) => n,
        };

        RegisterValue::Val64(val1 - val2)
    }
}

impl Mul for RegisterValue {
    type Output = RegisterValue;

    fn mul(self, other: RegisterValue) -> RegisterValue {
        let val1 = match self {
            Self::Val32(n) => n as u64,
            Self::Val64(n) => n,
        };

        let val2 = match other {
            Self::Val32(n) => n as u64,
            Self::Val64(n) => n,
        };

        RegisterValue::Val64(val1 * val2)
    }
}

impl BitAnd for RegisterValue {
    type Output = RegisterValue;

    fn bitand(self, other: RegisterValue) -> RegisterValue {
        let val1 = match self {
            Self::Val32(n) => n as u64,
            Self::Val64(n) => n,
        };

        let val2 = match other {
            Self::Val32(n) => n as u64,
            Self::Val64(n) => n,
        };

        RegisterValue::Val64(val1 & val2)
    }
}

impl RegisterValue {
    pub fn convert_32(&self) -> u32 {
        match self {
            Self::Val32(n) => *n,
            Self::Val64(n) => *n as u32,
        }
    }

    pub fn convert_64(&self) -> u64 {
        match self {
            Self::Val32(n) => *n as u64,
            Self::Val64(n) => *n,
        }
    }

    pub fn convert_reg(self, reg_name: &str) -> RegisterValue {
        if reg_name.chars().nth(0).unwrap() == 'W' {
            return RegisterValue::Val32(self.convert_32());
        }

        RegisterValue::Val64(self.convert_64())
    }
}

#[derive(Clone)]
pub struct Register {
    pub name: String,
    pub value: RegisterValue,
    pub bit_count: u16,
}

impl Register {
    pub fn new(name: &str, bit_count: u16) -> Register {
        let value: RegisterValue;

        if bit_count == 32 {
            value = RegisterValue::Val32(0);
        } else {
            value = RegisterValue::Val64(0);
        }

        Register {
            name: String::from(name),
            value: value,
            bit_count: bit_count,
        }
    }
}

pub fn create_registers() -> Vec<Register> {
    let mut registers: Vec<Register> = Vec::new();

    for i in 0..31 {
        registers.push(Register::new(&format!("X{}", i), 64));
    }

    for i in 0..31 {
        registers.push(Register::new(&format!("W{}", i), 32));
    }

    registers.push(Register::new("SP", 64));
    registers.push(Register::new("PC", 64));
    registers.push(Register::new("XZR", 64));
    registers.push(Register::new("WZR", 32));

    registers
}

pub fn get_register<'a>(registers: &'a Vec<Register>, name: &'a str) -> Option<&'a Register> {
    if name.len() < 2 {
        return None;
    }

    for i in registers {
        if i.name == name {
            return Some(i);
        }
    }

    None
}

pub fn set_register_value(registers: &mut Vec<Register>, name: &str, value: RegisterValue) {
    if name.len() < 2 || name == "XZR" || name == "WZR" {
        return;
    }

    let mut reg_name = String::from(name);
    let is_32_bit = reg_name.chars().nth(0).unwrap() == 'W';

    for i in registers.iter_mut() {
        if i.name == reg_name {
            i.value = value;
        }
    }

    if is_32_bit {
        reg_name.replace_range(0..1, "X");
    } else {
        reg_name.replace_range(0..1, "W");
    }

    for i in registers.iter_mut() {
        if i.name == reg_name {
            i.value = match value {
                RegisterValue::Val32(n) => RegisterValue::Val64(n as u64),
                RegisterValue::Val64(n) => RegisterValue::Val32(n as u32),
            };
        }
    }
}

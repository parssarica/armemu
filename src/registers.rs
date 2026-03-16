#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RegisterValue {
    Val32(u32),
    Val64(u64),
}

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
    if name.len() < 2 {
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

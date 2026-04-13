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
    Eor {
        op1: String,
        op2: String,
        op3: Operand,
    },
    Eon {
        op1: String,
        op2: String,
        op3: Operand,
    },
    Bic {
        op1: String,
        op2: String,
        op3: Operand,
    },
    Lsl {
        op1: String,
        op2: String,
        op3: Operand,
    },
    Lsr {
        op1: String,
        op2: String,
        op3: Operand,
    },
    Asr {
        op1: String,
        op2: String,
        op3: Operand,
    },
    Ror {
        op1: String,
        op2: String,
        op3: Operand,
    },
    Ubfx {
        op1: String,
        op2: String,
        op3: Operand,
        op4: Operand,
    },
    Sbfx {
        op1: String,
        op2: String,
        op3: Operand,
        op4: Operand,
    },
    Adc {
        op1: String,
        op2: String,
        op3: String,
    },
    Sbc {
        op1: String,
        op2: String,
        op3: String,
    },
    Neg {
        op1: String,
        op2: String,
    },
    Negs {
        op1: String,
        op2: String,
    },
    Ldrb {
        op1: String,
        op2: MemoryAddress,
    },
    Ldrh {
        op1: String,
        op2: MemoryAddress,
    },
    Ldrsw {
        op1: String,
        op2: MemoryAddress,
    },
    Strb {
        op1: String,
        op2: MemoryAddress,
    },
    Strh {
        op1: String,
        op2: MemoryAddress,
    },
    Ldp {
        op1: String,
        op2: String,
        op3: MemoryAddress,
    },
    Stp {
        op1: String,
        op2: String,
        op3: MemoryAddress,
    },
    Cbz {
        op1: String,
        op2: Operand,
    },
    Cbnz {
        op1: String,
        op2: Operand,
    },
    Nop,
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
        "eor" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            registers,
        )?,
        "eon" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            registers,
        )?,
        "bic" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            registers,
        )?,
        "lsl" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::Immediate),
            None,
            registers,
        )?,
        "lsr" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::Immediate),
            None,
            registers,
        )?,
        "asr" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::Immediate),
            None,
            registers,
        )?,
        "ror" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::Immediate),
            None,
            registers,
        )?,
        "ubfx" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::Immediate),
            Some(OperandType::Immediate),
            registers,
        )?,
        "sbfx" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::Immediate),
            Some(OperandType::Immediate),
            registers,
        )?,
        "adc" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::Register),
            None,
            registers,
        )?,
        "sbc" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::Register),
            None,
            registers,
        )?,
        "neg" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            None,
            None,
            registers,
        )?,
        "negs" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            None,
            None,
            registers,
        )?,
        "ldrb" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::MemoryAddress),
            None,
            None,
            registers,
        )?,
        "ldrh" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::MemoryAddress),
            None,
            None,
            registers,
        )?,
        "ldrsw" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::MemoryAddress),
            None,
            None,
            registers,
        )?,
        "strb" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::MemoryAddress),
            None,
            None,
            registers,
        )?,
        "strh" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::MemoryAddress),
            None,
            None,
            registers,
        )?,
        "ldp" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::MemoryAddress),
            None,
            registers,
        )?,
        "stp" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::Register),
            Some(OperandType::MemoryAddress),
            None,
            registers,
        )?,
        "cbz" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            None,
            registers,
        )?,
        "cbnz" => operand_check(
            ins,
            Some(OperandType::Register),
            Some(OperandType::RegImm),
            None,
            None,
            registers,
        )?,
        "nop" => operand_check(ins, None, None, None, None, registers)?,
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
            "eor" => Instructions::Eor {
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
            "eon" => Instructions::Eon {
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
            "bic" => Instructions::Bic {
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
            "lsl" => Instructions::Lsl {
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
            "lsr" => Instructions::Lsr {
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
            "asr" => Instructions::Asr {
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
            "ror" => Instructions::Ror {
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
            "ubfx" => Instructions::Ubfx {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op3: ins.op3.as_ref().unwrap().clone(),
                op4: ins.op4.as_ref().unwrap().clone(),
            },
            "sbfx" => Instructions::Sbfx {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op3: ins.op3.as_ref().unwrap().clone(),
                op4: ins.op4.as_ref().unwrap().clone(),
            },
            "adc" => Instructions::Adc {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op3: match ins.op3.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
            },
            "sbc" => Instructions::Sbc {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op3: match ins.op3.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
            },
            "neg" => Instructions::Neg {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
            },
            "negs" => Instructions::Negs {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
            },
            "ldrb" => Instructions::Ldrb {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandAddress(n) => n.clone(),
                    _ => unreachable!(),
                },
            },
            "ldrh" => Instructions::Ldrh {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandAddress(n) => n.clone(),
                    _ => unreachable!(),
                },
            },
            "ldrsw" => Instructions::Ldrsw {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandAddress(n) => n.clone(),
                    _ => unreachable!(),
                },
            },
            "strb" => Instructions::Strb {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandAddress(n) => n.clone(),
                    _ => unreachable!(),
                },
            },
            "strh" => Instructions::Strh {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandAddress(n) => n.clone(),
                    _ => unreachable!(),
                },
            },
            "ldp" => Instructions::Ldp {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op3: match ins.op3.as_ref().unwrap() {
                    Operand::OperandAddress(n) => n.clone(),
                    _ => unreachable!(),
                },
            },
            "stp" => Instructions::Stp {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: match ins.op2.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op3: match ins.op3.as_ref().unwrap() {
                    Operand::OperandAddress(n) => n.clone(),
                    _ => unreachable!(),
                },
            },
            "cbz" => Instructions::Cbz {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: ins.op2.as_ref().unwrap().clone(),
            },
            "cbnz" => Instructions::Cbnz {
                op1: match ins.op1.as_ref().unwrap() {
                    Operand::OperandRegister(n) => n.to_string(),
                    _ => unreachable!(),
                },
                op2: ins.op2.as_ref().unwrap().clone(),
            },
            "nop" => Instructions::Nop,
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

pub fn eor(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &Operand) {
    set_register_value(
        registers,
        op1,
        get_register_value(registers, op2).unwrap() ^ op3.convert_reg_val(registers).unwrap(),
    );
}

pub fn eon(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &Operand) {
    set_register_value(
        registers,
        op1,
        get_register_value(registers, op2).unwrap() ^ (!op3.convert_reg_val(registers).unwrap()),
    );
}

pub fn bic(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &Operand) {
    set_register_value(
        registers,
        op1,
        get_register_value(registers, op2).unwrap() & (!op3.convert_reg_val(registers).unwrap()),
    );
}

pub fn lsl(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &Operand) {
    set_register_value(
        registers,
        op1,
        get_register_value(registers, op2).unwrap() << op3.convert_reg_val(registers).unwrap(),
    );
}

pub fn lsr(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &Operand) {
    set_register_value(
        registers,
        op1,
        get_register_value(registers, op2).unwrap() >> op3.convert_reg_val(registers).unwrap(),
    );
}

pub fn asr(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &Operand) {
    let val = RegisterValue::Val32(
        (RegisterValue::Val32(get_register_value(registers, op2).unwrap().convert_32())
            >> RegisterValue::Val32(op3.convert_reg_val(registers).unwrap().convert_32()))
        .convert_32(),
    );
    set_register_value(
        registers,
        op1,
        match op1.chars().nth(0).unwrap() {
            'W' => val,
            _ => RegisterValue::Val64(val.convert_64()),
        },
    );
}

pub fn ror(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &Operand) {
    let val = get_register_value(registers, op2).unwrap();
    let n = op3.convert_reg_val(registers).unwrap();

    set_register_value(
        registers,
        op1,
        (val >> n)
            | (val
                << (RegisterValue::Val64(match op1.chars().nth(0).unwrap() {
                    'W' => 32,
                    _ => 64,
                }) - n)),
    );
}

pub fn ubfx(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &Operand, op4: &Operand) {
    let op2_val = get_register_value(registers, op2).unwrap();
    let bitstart = op3.convert_reg_val(registers).unwrap();
    let bitlength = op4.convert_reg_val(registers).unwrap();

    set_register_value(
        registers,
        op1,
        (op2_val >> bitstart) & ((RegisterValue::Val64(1) << bitlength) - RegisterValue::Val64(1)),
    );
}

pub fn sbfx(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &Operand, op4: &Operand) {
    let op2_val = get_register_value(registers, op2).unwrap();
    let lsb = op3.convert_reg_val(registers).unwrap();
    let width = op4.convert_reg_val(registers).unwrap();
    let bits = match op1.chars().nth(0).unwrap() {
        'W' => 32,
        _ => 64,
    };

    if bits == 64 {
        set_register_value(
            registers,
            op1,
            RegisterValue::Val64(
                (((op2_val << (RegisterValue::Val64(64) - width - lsb)).convert_64() as i64)
                    >> ((RegisterValue::Val64(64) - width).convert_64() as i64))
                    as u64,
            ),
        );
    } else {
        set_register_value(
            registers,
            op1,
            RegisterValue::Val32(
                (((op2_val << (RegisterValue::Val64(32) - width - lsb)).convert_32() as i32)
                    >> ((RegisterValue::Val64(32) - width).convert_32() as i32))
                    as u32,
            ),
        );
    }
}

pub fn adc(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &str) {
    set_register_value(
        registers,
        op1,
        get_register_value(registers, op2).unwrap()
            + get_register_value(registers, op3).unwrap()
            + if get_flag(registers, "C") {
                if op1.chars().nth(0).unwrap() == 'W' {
                    RegisterValue::Val32(1)
                } else {
                    RegisterValue::Val64(1)
                }
            } else {
                if op1.chars().nth(0).unwrap() == 'W' {
                    RegisterValue::Val32(0)
                } else {
                    RegisterValue::Val64(0)
                }
            },
    );
}

pub fn sbc(registers: &mut Vec<Register>, op1: &str, op2: &str, op3: &str) {
    let is_32_bit = op1.chars().nth(0).unwrap() == 'W';

    set_register_value(
        registers,
        op1,
        get_register_value(registers, op2).unwrap()
            - get_register_value(registers, op3).unwrap()
            - if is_32_bit {
                RegisterValue::Val32(1)
            } else {
                RegisterValue::Val64(1)
            }
            + if get_flag(registers, "C") {
                if is_32_bit {
                    RegisterValue::Val32(1)
                } else {
                    RegisterValue::Val64(1)
                }
            } else {
                if is_32_bit {
                    RegisterValue::Val32(0)
                } else {
                    RegisterValue::Val64(0)
                }
            },
    );
}

pub fn neg(registers: &mut Vec<Register>, op1: &str, op2: &str) {
    if op2.chars().nth(0).unwrap() == 'W' {
        set_register_value(
            registers,
            op1,
            RegisterValue::Val32(4294967295) - get_register_value(registers, op2).unwrap()
                + RegisterValue::Val32(1),
        );
    } else {
        set_register_value(
            registers,
            op1,
            RegisterValue::Val64(18446744073709551615)
                - get_register_value(registers, op2).unwrap()
                + RegisterValue::Val64(1),
        )
    }
}

pub fn negs(registers: &mut Vec<Register>, op1: &str, op2: &str) {
    neg(registers, op1, op2);
    let output = get_register_value(registers, op1).unwrap();
    let is_32_bit = op1.chars().nth(0).unwrap() == 'W';

    if is_32_bit {
        let output32 = output.convert_32();
        let input = get_register_value(registers, op2).unwrap().convert_32();
        if output32 >= 2147483648 {
            set_flag(registers, "N", true);
        } else {
            set_flag(registers, "N", false);
        }

        if output32 == 0 {
            set_flag(registers, "Z", true);
        } else {
            set_flag(registers, "Z", false);
        }

        if input == 0 {
            set_flag(registers, "C", true);
        } else {
            set_flag(registers, "C", false);
        }

        if input == 2147483648 {
            set_flag(registers, "V", true);
        } else {
            set_flag(registers, "V", false);
        }
    } else {
        let output64 = output.convert_64();
        let input = get_register_value(registers, op2).unwrap().convert_64();
        if output64 >= 9223372036854775808 {
            set_flag(registers, "N", true);
        } else {
            set_flag(registers, "N", false);
        }

        if output64 == 0 {
            set_flag(registers, "Z", true);
        } else {
            set_flag(registers, "Z", false);
        }

        if input == 0 {
            set_flag(registers, "C", true);
        } else {
            set_flag(registers, "C", false);
        }

        if input == 9223372036854775808 {
            set_flag(registers, "V", true);
        } else {
            set_flag(registers, "V", false);
        }
    }
}

pub fn ldrb(
    registers: &mut Vec<Register>,
    op1: &str,
    op2: MemoryAddress,
    memory: &Vec<u8>,
) -> Result<(), String> {
    let mem_addr_obj = {
        op2.change_reg_preindex(registers);
        op2
    };

    let addr = mem_addr_obj.get_addr(registers).convert_64();
    let byte = memory
        .get(addr as usize)
        .ok_or_else(|| String::from("Invalid memory address"))?;

    set_register_value(registers, op1, RegisterValue::Val64((*byte) as u64));

    mem_addr_obj.change_reg_postindex(registers);

    Ok(())
}

pub fn ldrh(
    registers: &mut Vec<Register>,
    op1: &str,
    op2: MemoryAddress,
    memory: &Vec<u8>,
) -> Result<(), String> {
    let mem_addr_obj = {
        op2.change_reg_preindex(registers);
        op2
    };

    let addr = mem_addr_obj.get_addr(registers).convert_64();
    let byte1 = *(memory
        .get(addr as usize)
        .ok_or_else(|| String::from("Invalid memory address"))?) as u64;
    let byte2 = *(memory
        .get((addr as usize) + 1)
        .ok_or_else(|| String::from("Invalid memory address"))?) as u64;

    set_register_value(registers, op1, RegisterValue::Val64((byte2 << 8) | byte1));

    mem_addr_obj.change_reg_postindex(registers);

    Ok(())
}

pub fn ldrsw(
    registers: &mut Vec<Register>,
    op1: &str,
    op2: MemoryAddress,
    memory: &Vec<u8>,
) -> Result<(), String> {
    let mem_addr_obj = {
        op2.change_reg_preindex(registers);
        op2
    };

    let addr = mem_addr_obj.get_addr(registers).convert_64() as usize;

    if addr + 4 > memory.len() {
        return Err(String::from("Invalid memory address"));
    }

    let mem_bytes: &[u8] = &memory[addr..addr + 4];

    let num = i32::from_le_bytes(mem_bytes[..4].try_into().unwrap());

    set_register_value(registers, op1, RegisterValue::Val64(num as u64));

    mem_addr_obj.change_reg_postindex(registers);

    Ok(())
}

pub fn strb(
    registers: &mut Vec<Register>,
    op1: &str,
    op2: MemoryAddress,
    memory: &mut Vec<u8>,
) -> Result<(), String> {
    let mem_addr_obj = {
        op2.change_reg_preindex(registers);
        op2
    };

    let addr = mem_addr_obj.get_addr(registers).convert_64() as usize;
    let byte = get_register_value(registers, op1).unwrap().convert_64() as u8;

    if addr >= memory.len() {
        return Err(String::from("Invalid memory address"));
    }

    memory[addr] = byte;

    mem_addr_obj.change_reg_postindex(registers);

    Ok(())
}

pub fn strh(
    registers: &mut Vec<Register>,
    op1: &str,
    op2: MemoryAddress,
    memory: &mut Vec<u8>,
) -> Result<(), String> {
    let mem_addr_obj = {
        op2.change_reg_preindex(registers);
        op2
    };

    let addr = mem_addr_obj.get_addr(registers).convert_64() as usize;
    let val = get_register_value(registers, op1).unwrap().convert_64();
    let byte1 = (val & 0xff) as u8;
    let byte2 = ((val & 0xff00) >> 8) as u8;

    if addr + 1 >= memory.len() {
        return Err(String::from("Invalid memory address."));
    }

    memory[addr] = byte1;
    memory[addr + 1] = byte2;

    mem_addr_obj.change_reg_postindex(registers);

    Ok(())
}

pub fn ldp(
    registers: &mut Vec<Register>,
    op1: &str,
    op2: &str,
    op3: MemoryAddress,
    memory: &Vec<u8>,
) -> Result<(), String> {
    let mem_addr_obj = {
        op3.change_reg_preindex(registers);
        op3
    };

    let addr = mem_addr_obj.get_addr(registers).convert_64() as usize;

    if addr + 16 > memory.len() {
        return Err(String::from("Invalid memory address."));
    }

    if op1.chars().nth(0).unwrap() == 'W' {
        let val1 = u32::from_le_bytes((&memory[addr..addr + 4]).try_into().unwrap());
        let val2 = u32::from_le_bytes((&memory[addr + 4..addr + 8]).try_into().unwrap());
        set_register_value(registers, op1, RegisterValue::Val32(val1));
        set_register_value(registers, op2, RegisterValue::Val32(val2));
    } else {
        let val1 = u64::from_le_bytes((&memory[addr..addr + 8]).try_into().unwrap());
        let val2 = u64::from_le_bytes((&memory[addr + 8..addr + 16]).try_into().unwrap());
        set_register_value(registers, op1, RegisterValue::Val64(val1));
        set_register_value(registers, op2, RegisterValue::Val64(val2));
    }

    mem_addr_obj.change_reg_postindex(registers);

    Ok(())
}

pub fn stp(
    registers: &mut Vec<Register>,
    op1: &str,
    op2: &str,
    op3: MemoryAddress,
    memory: &mut Vec<u8>,
) -> Result<(), String> {
    let mem_addr_obj = {
        op3.change_reg_preindex(registers);
        op3
    };

    let addr = mem_addr_obj.get_addr(registers).convert_64() as usize;

    if addr + 16 > memory.len() {
        return Err(String::from("Invalid memory address."));
    }

    if op1.chars().nth(0).unwrap() == 'W' {
        let val1 = get_register_value(registers, op1)
            .unwrap()
            .convert_32()
            .to_le_bytes();
        let val2 = get_register_value(registers, op2)
            .unwrap()
            .convert_32()
            .to_le_bytes();
        memory[addr..addr + 4].copy_from_slice(val1.as_slice());
        memory[addr + 4..addr + 8].copy_from_slice(val2.as_slice());
    } else {
        let val1 = get_register_value(registers, op1)
            .unwrap()
            .convert_64()
            .to_le_bytes();
        let val2 = get_register_value(registers, op2)
            .unwrap()
            .convert_64()
            .to_le_bytes();
        memory[addr..addr + 8].copy_from_slice(val1.as_slice());
        memory[addr + 8..addr + 16].copy_from_slice(val2.as_slice());
    }

    mem_addr_obj.change_reg_postindex(registers);

    Ok(())
}

pub fn cbz(registers: &mut Vec<Register>, op1: &str, op2: &Operand) {
    if get_register_value(registers, op1).unwrap().convert_64() == 0 {
        let offset = op2.convert_reg_val(registers).unwrap().convert_64() as i64;
        let pc = get_register_value(registers, "PC").unwrap().convert_64() as i64;

        let new_pc = pc + offset - 4;

        set_register_value(registers, "PC", RegisterValue::Val64(new_pc as u64));
    }
}

pub fn cbnz(registers: &mut Vec<Register>, op1: &str, op2: &Operand) {
    if get_register_value(registers, op1).unwrap().convert_64() != 0 {
        let offset = op2.convert_reg_val(registers).unwrap().convert_64() as i64;
        let pc = get_register_value(registers, "PC").unwrap().convert_64() as i64;

        let new_pc = pc + offset - 4;

        set_register_value(registers, "PC", RegisterValue::Val64(new_pc as u64));
    }
}

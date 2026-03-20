#[cfg(test)]
mod tests {
    use crate::instruction_parser::*;
    use crate::instructions::*;
    use crate::parse_toml::*;
    use crate::registers::*;

    #[test]
    fn create_register_64bit() {
        let x = Register::new("REGISTER", 64);

        assert_eq!(x.name, "REGISTER", "Register names didn't match.");
        assert_eq!(x.bit_count, 64, "Bit count didn't match.");
        assert_eq!(x.value, RegisterValue::Val64(0), "Value was unexpected.");
    }

    #[test]
    fn create_register_32bit() {
        let x = Register::new("REGISTER", 32);

        assert_eq!(x.name, "REGISTER", "Register names didn't match.");
        assert_eq!(x.bit_count, 32, "Bit count didn't match.");
        assert_eq!(x.value, RegisterValue::Val32(0), "Value was unexpected.");
    }

    #[test]
    fn create_registers_test() {
        let registers = create_registers();
        let mut i = 0;

        assert_eq!(registers.len(), 66, "Not enough registers was created.");
        for _ in &registers {
            if i < 31 {
                assert_eq!(
                    registers[i].name,
                    format!("X{}", i),
                    "Register name didn't match."
                );
            } else if i < 62 {
                assert_eq!(
                    registers[i].name,
                    format!("W{}", i - 31),
                    "Register name didn't match."
                );
            } else if i == 62 {
                assert_eq!(registers[i].name, "SP", "Register name didn't match.");
            } else if i == 63 {
                assert_eq!(registers[i].name, "PC", "Register name didn't match.");
            } else if i == 64 {
                assert_eq!(registers[i].name, "XZR", "Register name didn't match.");
            } else {
                assert_eq!(registers[i].name, "WZR", "Register name didn't match.");
            }

            if (i > 30 && i < 62) || (i == 65) {
                assert_eq!(registers[i].value, RegisterValue::Val32(0));
                assert_eq!(registers[i].bit_count, 32);
            } else {
                assert_eq!(registers[i].value, RegisterValue::Val64(0));
                assert_eq!(registers[i].bit_count, 64);
            }

            i += 1;
        }
    }

    #[test]
    fn get_register_test() {
        let registers = create_registers();
        let register1 = get_register(&registers, "X0").unwrap();
        let register2 = get_register(&registers, "W0").unwrap();
        let register3 = get_register(&registers, "NONEXISTINGREGISTER");

        assert_eq!(register1.name, "X0", "Register name didn't match.");
        assert_eq!(
            register1.value,
            RegisterValue::Val64(0),
            "Register value was unexpected."
        );
        assert_eq!(register1.bit_count, 64, "Register value was unexpected.");
        assert_eq!(register2.name, "W0", "Register name didn't match.");
        assert_eq!(
            register2.value,
            RegisterValue::Val32(0),
            "Register value was unexpected."
        );
        assert_eq!(register2.bit_count, 32, "Register value was unexpected.");
        assert!(register3.is_none(), "A non-existing register was found.");
    }

    #[test]
    fn set_register_value_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X0", RegisterValue::Val64(1));
        set_register_value(&mut registers, "W1", RegisterValue::Val32(2));
        assert_eq!(
            registers[0].value,
            RegisterValue::Val64(1),
            "Register value didn't match."
        );
        assert_eq!(
            registers[31].value,
            RegisterValue::Val32(1),
            "Register value didn't match."
        );
        assert_eq!(
            registers[1].value,
            RegisterValue::Val64(2),
            "Register value didn't match."
        );
        assert_eq!(
            registers[32].value,
            RegisterValue::Val32(2),
            "Register value didn't match."
        );
    }

    #[test]
    fn parse_instruction_test1() {
        let registers = create_registers();
        let line = "ADD X0, X1, X2, LSL #3";

        let ins = parse_instruction(line, &registers).unwrap();

        assert_eq!(ins.name, "ADD", "Instruction 1 name didn't match.");
        assert!(ins.op1.is_some(), "Instruction 1 operand 1 not found.");
        assert!(ins.op2.is_some(), "Instruction 1 operand 2 not found.");
        assert!(ins.op3.is_some(), "Instruction 1 operand 3 not found.");
        assert!(ins.op4.is_none(), "Instruction 1 operand 4 found.");
        assert!(
            matches!(ins.barrelshifter.clone(), Some(_)),
            "Instruction 1 barrel shifter not found."
        );
        assert!(
            matches!(
                ins.barrelshifter.clone().unwrap().barrelshiftertype,
                BarrelShifterType::LSL
            ),
            "Instruction 1 barrel shifter type didn't match."
        );
        assert!(
            matches!(ins.barrelshifter.clone().unwrap().value, Some(_)),
            "Instruction 1 barrel shifter value was not found."
        );
        assert!(
            matches!(
                ins.barrelshifter.clone().unwrap().value.unwrap(),
                RegisterValue::Val64(_)
            ),
            "Instruction 1 barrel shifter value was not found."
        );
        assert_eq!(
            ins.barrelshifter.clone().unwrap().value.unwrap(),
            RegisterValue::Val64(3),
            "Instruction 1 barrel shifter value was not found."
        );
    }

    #[test]
    fn parse_instruction_test2() {
        let registers = create_registers();
        let line = "SUBS X0, X1, X2, ASR #2";
        let ins = parse_instruction(line, &registers).unwrap();

        assert_eq!(ins.name, "SUBS", "Instruction 2 name didn't match.");
        assert!(ins.op1.is_some(), "Instruction 2 operand 1 not found.");
        assert!(ins.op2.is_some(), "Instruction 2 operand 2 not found.");
        assert!(ins.op3.is_some(), "Instruction 2 operand 3 not found.");
        assert!(ins.op4.is_none(), "Instruction 2 operand 4 found.");
        assert!(
            matches!(ins.barrelshifter.clone(), Some(_)),
            "Instruction 2 barrel shifter not found."
        );
        assert!(
            matches!(
                ins.barrelshifter.clone().unwrap().barrelshiftertype,
                BarrelShifterType::ASR
            ),
            "Instruction 2 barrel shifter type didn't match."
        );
        assert!(
            matches!(ins.barrelshifter.clone().unwrap().value, Some(_)),
            "Instruction 2 barrel shifter value was not found."
        );
        assert!(
            matches!(
                ins.barrelshifter.clone().unwrap().value.unwrap(),
                RegisterValue::Val64(_)
            ),
            "Instruction 2 barrel shifter value was not found."
        );
        assert_eq!(
            ins.barrelshifter.clone().unwrap().value.unwrap(),
            RegisterValue::Val64(2),
            "Instruction 2 barrel shifter value was not found."
        );
    }

    #[test]
    fn parse_instruction_test3() {
        let registers = create_registers();
        let line = "MOV X0, X1, X2, ASR #3";
        let ins = parse_instruction(line, &registers).unwrap();
        assert_eq!(ins.name, "MOV", "Instruction 3 name didn't match.");
        assert!(ins.op1.is_some(), "Instruction 3 operand 1 not found.");
        assert!(ins.op2.is_some(), "Instruction 3 operand 2 not found.");
        assert!(ins.op3.is_some(), "Instruction 3 operand 3 not found.");
        assert!(ins.op4.is_none(), "Instruction 3 operand 4 found.");
        assert!(
            matches!(ins.barrelshifter.clone(), Some(_)),
            "Instruction 3 barrel shifter not found."
        );
        assert!(
            matches!(
                ins.barrelshifter.clone().unwrap().barrelshiftertype,
                BarrelShifterType::ASR
            ),
            "Instruction 3 barrel shifter type didn't match."
        );
        assert!(
            matches!(ins.barrelshifter.clone().unwrap().value, Some(_)),
            "Instruction 3 barrel shifter value was not found."
        );
        assert!(
            matches!(
                ins.barrelshifter.clone().unwrap().value.unwrap(),
                RegisterValue::Val64(_)
            ),
            "Instruction 3 barrel shifter value was not found."
        );
        assert_eq!(
            ins.barrelshifter.clone().unwrap().value.unwrap(),
            RegisterValue::Val64(3),
            "Instruction 3 barrel shifter value was not found."
        );
    }

    #[test]
    fn parse_instruction_test4() {
        let registers = create_registers();
        let line = "ADD X0, X1, X2";
        let ins = parse_instruction(line, &registers).unwrap();

        assert_eq!(ins.name, "ADD", "Instruction 4 name didn't match.");
        assert!(ins.op1.is_some(), "Instruction 4 operand 1 not found.");
        assert!(ins.op2.is_some(), "Instruction 4 operand 2 not found.");
        assert!(ins.op3.is_some(), "Instruction 4 operand 3 not found.");
        assert!(ins.op4.is_none(), "Instruction 4 operand 4 found.");
        assert!(
            matches!(ins.barrelshifter.clone(), None),
            "Instruction 4 barrel shifter found."
        );
    }

    #[test]
    fn parse_instruction_test5() {
        let registers = create_registers();
        let line = "MOV X1, #10";
        let ins = parse_instruction(line, &registers).unwrap();

        assert_eq!(ins.name, "MOV", "Instruction 5 name didn't match.");
        assert!(ins.op1.is_some(), "Instruction 5 operand 1 not found.");
        assert!(ins.op2.is_some(), "Instruction 5 operand 2 not found.");
        assert!(ins.op3.is_none(), "Instruction 5 operand 3 found.");
        assert!(ins.op4.is_none(), "Instruction 5 operand 4 found.");
        assert!(
            matches!(ins.barrelshifter.clone(), None),
            "Instruction 5 barrel shifter found."
        );
    }

    #[test]
    fn parse_instruction_test6() {
        let registers = create_registers();
        let line = "SUBS X1, X1, #1";
        let ins = parse_instruction(line, &registers).unwrap();

        assert_eq!(ins.name, "SUBS", "Instruction 6 name didn't match.");
        assert!(ins.op1.is_some(), "Instruction 6 operand 1 not found.");
        assert!(ins.op2.is_some(), "Instruction 6 operand 2 not found.");
        assert!(ins.op3.is_some(), "Instruction 6 operand 3 found.");
        assert!(ins.op4.is_none(), "Instruction 6 operand 4 found.");
        assert!(
            matches!(ins.barrelshifter.clone(), None),
            "Instruction 6 barrel shifter found."
        );
    }

    #[test]
    fn parse_instruction_test7() {
        let registers = create_registers();
        let line = "BX X30";
        let ins = parse_instruction(line, &registers).unwrap();

        assert_eq!(ins.name, "BX", "Instruction 7 name didn't match.");
        assert!(ins.op1.is_some(), "Instruction 7 operand 1 not found.");
        assert!(ins.op2.is_none(), "Instruction 7 operand 2 found.");
        assert!(ins.op3.is_none(), "Instruction 7 operand 3 found.");
        assert!(ins.op4.is_none(), "Instruction 7 operand 4 found.");
        assert!(
            matches!(ins.barrelshifter.clone(), None),
            "Instruction 7 barrel shifter found."
        );
    }

    #[test]
    fn parse_memory_test() {
        let size1 = parse_memory("100 GB");
        let size2 = parse_memory("100GB");
        let size3 = parse_memory("100 MB");
        let size4 = parse_memory("100MB");
        let size5 = parse_memory("100 KB");
        let size6 = parse_memory("100KB");
        let size7 = parse_memory("100");

        assert_eq!(size1, 107374182400, "Size 1 was wrong.");
        assert_eq!(size2, 107374182400, "Size 2 was wrong.");
        assert_eq!(size3, 104857600, "Size 3 was wrong.");
        assert_eq!(size4, 104857600, "Size 4 was wrong.");
        assert_eq!(size5, 102400, "Size 5 was wrong.");
        assert_eq!(size6, 102400, "Size 6 was wrong.");
        assert_eq!(size7, 100, "Size 7 was wrong.");
    }

    #[test]
    fn mov_test() {
        let mut registers = create_registers();
        let ins = Instruction {
            name: String::from("MOV"),
            op1: Some(Operand::OperandRegister(String::from("X0"))),
            op2: Some(Operand::OperandNumber(RegisterValue::Val64(314))),
            op3: None,
            op4: None,
            barrelshifter: None,
            operand_count: 2,
        };

        let mut res1 = Ok(());
        mov(&mut registers, &ins, &mut res1);

        assert!(res1.is_ok(), "Instruction failed for no reason.");
        assert_eq!(
            get_register(&registers, "X0").unwrap().value,
            RegisterValue::Val64(314),
            "Value was not moved by instruction."
        );
    }

    #[test]
    fn add_test() {
        let mut registers = create_registers();
        let ins = Instruction {
            name: String::from("ADD"),
            op1: Some(Operand::OperandRegister(String::from("X0"))),
            op2: Some(Operand::OperandRegister(String::from("X1"))),
            op3: Some(Operand::OperandNumber(RegisterValue::Val64(314))),
            op4: None,
            barrelshifter: None,
            operand_count: 3,
        };

        set_register_value(&mut registers, "X1", RegisterValue::Val64(2827));

        let mut res1 = Ok(());
        add(&mut registers, &ins, &mut res1);

        assert!(res1.is_ok(), "Instruction failed for no reason.");
        assert_eq!(
            get_register(&registers, "X0").unwrap().value,
            RegisterValue::Val64(3141),
            "Value was not added by instruction."
        );
    }

    #[test]
    fn sub_test() {
        let mut registers = create_registers();
        let ins = Instruction {
            name: String::from("SUB"),
            op1: Some(Operand::OperandRegister(String::from("X0"))),
            op2: Some(Operand::OperandRegister(String::from("X1"))),
            op3: Some(Operand::OperandNumber(RegisterValue::Val64(314))),
            op4: None,
            barrelshifter: None,
            operand_count: 3,
        };

        set_register_value(&mut registers, "X1", RegisterValue::Val64(3455));

        let mut res1 = Ok(());
        sub(&mut registers, &ins, &mut res1);

        assert!(res1.is_ok(), "Instruction failed for no reason.");
        assert_eq!(
            get_register(&registers, "X0").unwrap().value,
            RegisterValue::Val64(3141),
            "Value was not added by instruction."
        );
    }
}

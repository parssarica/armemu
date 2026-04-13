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

        assert_eq!(registers.len(), 67, "Not enough registers was created.");
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
            } else if i == 65 {
                assert_eq!(registers[i].name, "WZR", "Register name didn't match.");
            } else if i == 66 {
                assert_eq!(registers[i].name, "NZCV", "Register name didn't match.");
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

        let ins = parse_instruction(line, &registers, &Vec::<(&str, u64)>::new(), 0).unwrap();

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
        let ins = parse_instruction(line, &registers, &Vec::<(&str, u64)>::new(), 0).unwrap();

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
        let ins = parse_instruction(line, &registers, &Vec::<(&str, u64)>::new(), 0).unwrap();
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
        let ins = parse_instruction(line, &registers, &Vec::<(&str, u64)>::new(), 0).unwrap();

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
        let ins = parse_instruction(line, &registers, &Vec::<(&str, u64)>::new(), 0).unwrap();

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
        let ins = parse_instruction(line, &registers, &Vec::<(&str, u64)>::new(), 0).unwrap();

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
        let ins = parse_instruction(line, &registers, &Vec::<(&str, u64)>::new(), 0).unwrap();

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

        let converted = convert_ins(&ins, &registers).expect("Conversion failed for no reason.");
        match converted {
            Instructions::Mov { ref op1, op2 } => mov(&mut registers, op1, op2),
            _ => panic!("convert_ins converted wrongly."),
        }

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

        let converted = convert_ins(&ins, &registers).expect("Conversion failed for no reason.");
        match converted {
            Instructions::Add {
                ref op1,
                ref op2,
                op3,
            } => add(&mut registers, op1, op2, op3),
            _ => panic!("convert_ins converted wrongly."),
        }

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

        let converted = convert_ins(&ins, &registers).expect("Conversion failed for no reason.");
        match converted {
            Instructions::Sub {
                ref op1,
                ref op2,
                op3,
            } => sub(&mut registers, op1, op2, op3),
            _ => panic!("convert_ins converted wrongly."),
        }

        assert_eq!(
            get_register(&registers, "X0").unwrap().value,
            RegisterValue::Val64(3141),
            "Value was not added by instruction."
        );
    }

    #[test]
    fn mul_test() {
        let mut registers = create_registers();
        let ins = Instruction {
            name: String::from("MUL"),
            op1: Some(Operand::OperandRegister(String::from("X0"))),
            op2: Some(Operand::OperandRegister(String::from("X1"))),
            op3: Some(Operand::OperandRegister(String::from("X2"))),
            op4: None,
            barrelshifter: None,
            operand_count: 3,
        };

        set_register_value(&mut registers, "X1", RegisterValue::Val64(349));
        set_register_value(&mut registers, "X2", RegisterValue::Val64(9));

        let converted = convert_ins(&ins, &registers).expect("Conversion failed for no reason.");
        match converted {
            Instructions::Mul {
                ref op1,
                ref op2,
                op3,
            } => mul(&mut registers, op1, op2, op3),
            _ => panic!("convert_ins converted wrongly."),
        }

        assert_eq!(
            get_register(&registers, "X0").unwrap().value,
            RegisterValue::Val64(3141),
            "Value was not added by instruction."
        );
    }

    #[test]
    fn and_test() {
        let mut registers = create_registers();
        let ins = Instruction {
            name: String::from("AND"),
            op1: Some(Operand::OperandRegister(String::from("X0"))),
            op2: Some(Operand::OperandRegister(String::from("X1"))),
            op3: Some(Operand::OperandNumber(RegisterValue::Val64(12))),
            op4: None,
            barrelshifter: None,
            operand_count: 3,
        };

        set_register_value(&mut registers, "X1", RegisterValue::Val64(24));

        let converted = convert_ins(&ins, &registers).expect("Conversion failed for no reason.");
        match converted {
            Instructions::And {
                ref op1,
                ref op2,
                op3,
            } => and(&mut registers, op1, op2, op3),
            _ => panic!("convert_ins converted wrongly."),
        }

        assert_eq!(
            get_register(&registers, "X0").unwrap().value,
            RegisterValue::Val64(8),
            "Value was not added by instruction."
        );
    }

    #[test]
    fn set_flag_test() {
        let mut registers = create_registers();

        set_flag(&mut registers, "N", true);
        assert_eq!(
            get_register(&registers, "NZCV").unwrap().value,
            RegisterValue::Val64(2147483648),
            "N flag was not set."
        );
        set_flag(&mut registers, "Z", true);
        assert_eq!(
            get_register(&registers, "NZCV").unwrap().value,
            RegisterValue::Val64(3221225472),
            "Z flag was not set."
        );
        set_flag(&mut registers, "C", true);
        assert_eq!(
            get_register(&registers, "NZCV").unwrap().value,
            RegisterValue::Val64(3758096384),
            "C flag was not set."
        );
        set_flag(&mut registers, "V", true);
        assert_eq!(
            get_register(&registers, "NZCV").unwrap().value,
            RegisterValue::Val64(4026531840),
            "V flag was not set."
        );
        set_flag(&mut registers, "N", false);
        assert_eq!(
            get_register(&registers, "NZCV").unwrap().value,
            RegisterValue::Val64(1879048192),
            "N flag was not cleared."
        );
        set_flag(&mut registers, "Z", false);
        assert_eq!(
            get_register(&registers, "NZCV").unwrap().value,
            RegisterValue::Val64(805306368),
            "Z flag was not cleared."
        );
        set_flag(&mut registers, "C", false);
        assert_eq!(
            get_register(&registers, "NZCV").unwrap().value,
            RegisterValue::Val64(268435456),
            "C flag was not cleared."
        );
        set_flag(&mut registers, "V", false);
        assert_eq!(
            get_register(&registers, "NZCV").unwrap().value,
            RegisterValue::Val64(0),
            "V flag was not cleared."
        );
    }

    #[test]
    fn get_flag_test() {
        let mut registers = create_registers();

        set_flag(&mut registers, "N", true);
        assert_eq!(get_flag(&registers, "N"), true, "N flag was not set.");
        set_flag(&mut registers, "Z", true);
        assert_eq!(get_flag(&registers, "Z"), true, "Z flag was not set.");
        set_flag(&mut registers, "C", true);
        assert_eq!(get_flag(&registers, "C"), true, "C flag was not set.");
        set_flag(&mut registers, "V", true);
        assert_eq!(get_flag(&registers, "V"), true, "V flag was not set.");
        set_flag(&mut registers, "N", false);
        assert_eq!(get_flag(&registers, "N"), false, "N flag was not cleared.");
        set_flag(&mut registers, "Z", false);
        assert_eq!(get_flag(&registers, "Z"), false, "Z flag was not cleared.");
        set_flag(&mut registers, "C", false);
        assert_eq!(get_flag(&registers, "C"), false, "C flag was not cleared.");
        set_flag(&mut registers, "V", false);
        assert_eq!(get_flag(&registers, "V"), false, "V flag was not cleared.");
    }

    #[test]
    fn get_register_value_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X0", RegisterValue::Val64(10));
        assert_eq!(
            get_register_value(&registers, "X0").unwrap(),
            RegisterValue::Val64(10),
            "X0's value got wrong."
        );
        set_register_value(&mut registers, "X0", RegisterValue::Val64(20));
        assert_eq!(
            get_register_value(&registers, "X0").unwrap(),
            RegisterValue::Val64(20),
            "X0's value got wrong."
        );
        set_register_value(&mut registers, "W0", RegisterValue::Val32(10));
        assert_eq!(
            get_register_value(&registers, "W0").unwrap(),
            RegisterValue::Val32(10),
            "W0's value got wrong."
        );
        set_register_value(&mut registers, "W0", RegisterValue::Val32(20));
        assert_eq!(
            get_register_value(&registers, "W0").unwrap(),
            RegisterValue::Val32(20),
            "W0's value got wrong."
        );
        set_register_value(&mut registers, "XZR", RegisterValue::Val64(10));
        assert_eq!(
            get_register_value(&registers, "XZR").unwrap(),
            RegisterValue::Val64(0),
            "XZR's value got wrong."
        );
        set_register_value(&mut registers, "XZR", RegisterValue::Val64(20));
        assert_eq!(
            get_register_value(&registers, "XZR").unwrap(),
            RegisterValue::Val64(0),
            "XZR's value got wrong."
        );
        set_register_value(&mut registers, "WZR", RegisterValue::Val32(10));
        assert_eq!(
            get_register_value(&registers, "WZR").unwrap(),
            RegisterValue::Val32(0),
            "WZR's value got wrong."
        );
        assert_eq!(
            get_register_value(&registers, "NONEXISTINGREGISTER"),
            None,
            "Non existing register was found."
        );
    }

    #[test]
    fn magic_split_test() {
        let line1 = "LDR X0, [X1]";
        let line2 = "LDR X0, [X1, #12]";
        let line3 = "LDR X0, [X1], #12";
        let line4 = "LDR X0, [X1, #12, LSL #23]";
        let line5 = "LDR X0, [X1, LSL #24], #12";
        let line6 = "";
        let line7 = "NOP";

        let output1 = magic_split(line1);
        let output2 = magic_split(line2);
        let output3 = magic_split(line3);
        let output4 = magic_split(line4);
        let output5 = magic_split(line5);
        let output6 = magic_split(line6);
        let output7 = magic_split(line7);

        assert!(output1.is_some(), "Split #1 failed.");
        assert!(output2.is_some(), "Split #2 failed.");
        assert!(output3.is_some(), "Split #3 failed.");
        assert!(output4.is_some(), "Split #4 failed.");
        assert!(output5.is_some(), "Split #5 failed.");
        assert!(output6.is_none(), "Split #6 didn't fail.");
        assert!(output7.is_some(), "Split #7 failed.");

        assert_eq!(
            output1.unwrap(),
            vec!["LDR".to_string(), "X0".to_string(), "[X1]".to_string()],
            "Split #1 worked wrong."
        );
        assert_eq!(
            output2.unwrap(),
            vec!["LDR".to_string(), "X0".to_string(), "[X1, #12]".to_string()],
            "Split #2 worked wrong."
        );
        assert_eq!(
            output3.unwrap(),
            vec!["LDR".to_string(), "X0".to_string(), "[X1], #12".to_string()],
            "Split #3 worked wrong."
        );
        assert_eq!(
            output4.unwrap(),
            vec![
                "LDR".to_string(),
                "X0".to_string(),
                "[X1, #12, LSL #23]".to_string()
            ],
            "Split #4 worked wrong."
        );
        assert_eq!(
            output5.unwrap(),
            vec![
                "LDR".to_string(),
                "X0".to_string(),
                "[X1, LSL #24], #12".to_string()
            ],
            "Split #5 worked wrong."
        );
        assert_eq!(
            output7.unwrap(),
            vec!["NOP".to_string(),],
            "Split #7 worked wrong."
        );
    }

    #[test]
    fn group_couple_test() {
        let g1 = group_couple("ABCD");
        let g2 = group_couple("ABCDE");
        let g3 = group_couple("");
        let g4 = group_couple(" a bc");

        assert_eq!(
            g1,
            vec!["AB".to_string(), "CD".to_string()],
            "Grouping #1 failed."
        );
        assert_eq!(
            g2,
            vec!["AB".to_string(), "CD".to_string(), "E".to_string()],
            "Grouping #2 failed."
        );
        assert_eq!(g3, Vec::<String>::new(), "Grouping #3 failed.");
        assert_eq!(
            g4,
            vec![" a".to_string(), " b".to_string(), "c".to_string()],
            "Grouping #4 failed."
        );
    }

    #[test]
    fn str_test() {
        let mut registers = create_registers();
        let mut memory: Vec<u8> = vec![0; 1024];
        let ins1 = Instruction {
            name: String::from("STR"),
            op1: Some(Operand::OperandRegister(String::from("X0"))),
            op2: Some(Operand::OperandAddress(MemoryAddress {
                base_address: String::from("X1"),
                second_val: None,
                barrelshifter: None,
                postindexval: None,
                addr_type: MemoryAddressType::Normal,
            })),
            op3: None,
            op4: None,
            barrelshifter: None,
            operand_count: 2,
        };
        let ins2 = Instruction {
            name: String::from("STR"),
            op1: Some(Operand::OperandRegister(String::from("X0"))),
            op2: Some(Operand::OperandAddress(MemoryAddress {
                base_address: String::from("X1"),
                second_val: None,
                barrelshifter: None,
                postindexval: Some(RegisterValue::Val64(58)),
                addr_type: MemoryAddressType::Postindexed,
            })),
            op3: None,
            op4: None,
            barrelshifter: None,
            operand_count: 2,
        };

        set_register_value(&mut registers, "X0", RegisterValue::Val64(314));
        set_register_value(&mut registers, "X1", RegisterValue::Val64(512));

        let converted = convert_ins(&ins1, &registers).expect("Conversion failed for no reason.");
        match converted {
            Instructions::Str { ref op1, op2 } => str(&mut registers, op1, op2, &mut memory)
                .expect("Instruction failed for no reason."),
            _ => panic!("convert_ins converted wrongly."),
        }

        assert_eq!(memory[512], 58, "Byte #1 didn't match.");
        assert_eq!(memory[513], 1, "Byte #2 didn't match.");
        assert_eq!(memory[514], 0, "Byte #3 didn't match.");
        assert_eq!(memory[515], 0, "Byte #4 didn't match.");
        assert_eq!(memory[516], 0, "Byte #5 didn't match.");
        assert_eq!(memory[517], 0, "Byte #6 didn't match.");
        assert_eq!(memory[518], 0, "Byte #7 didn't match.");
        assert_eq!(memory[519], 0, "Byte #8 didn't match.");

        set_register_value(&mut registers, "X1", RegisterValue::Val64(256));

        let converted = convert_ins(&ins2, &registers).expect("Conversion failed for no reason.");
        match converted {
            Instructions::Str { ref op1, op2 } => str(&mut registers, op1, op2, &mut memory)
                .expect("Instruction failed for no reason."),
            _ => panic!("convert_ins converted wrongly."),
        }

        assert_eq!(memory[256], 58, "Byte #9 didn't match.");
        assert_eq!(memory[257], 1, "Byte #10 didn't match.");
        assert_eq!(memory[258], 0, "Byte #11 didn't match.");
        assert_eq!(memory[259], 0, "Byte #12 didn't match.");
        assert_eq!(memory[260], 0, "Byte #13 didn't match.");
        assert_eq!(memory[261], 0, "Byte #14 didn't match.");
        assert_eq!(memory[262], 0, "Byte #15 didn't match.");
        assert_eq!(memory[263], 0, "Byte #16 didn't match.");

        assert_eq!(
            get_register_value(&registers, "X1").unwrap(),
            RegisterValue::Val64(314),
            "Post indexing not worked."
        );
    }

    #[test]
    fn ldr_test() {
        let mut registers = create_registers();
        let mut memory: Vec<u8> = vec![0; 1024];
        let ins = Instruction {
            name: String::from("STR"),
            op1: Some(Operand::OperandRegister(String::from("X0"))),
            op2: Some(Operand::OperandAddress(MemoryAddress {
                base_address: String::from("X1"),
                second_val: None,
                barrelshifter: None,
                postindexval: None,
                addr_type: MemoryAddressType::Normal,
            })),
            op3: None,
            op4: None,
            barrelshifter: None,
            operand_count: 2,
        };

        let ins2 = Instruction {
            name: String::from("LDR"),
            op1: Some(Operand::OperandRegister(String::from("X0"))),
            op2: Some(Operand::OperandAddress(MemoryAddress {
                base_address: String::from("X1"),
                second_val: None,
                barrelshifter: None,
                postindexval: Some(RegisterValue::Val64(58)),
                addr_type: MemoryAddressType::Postindexed,
            })),
            op3: None,
            op4: None,
            barrelshifter: None,
            operand_count: 2,
        };

        let ins3 = Instruction {
            name: String::from("LDR"),
            op1: Some(Operand::OperandRegister(String::from("X0"))),
            op2: Some(Operand::OperandNumber(RegisterValue::Val64(267))),
            op3: None,
            op4: None,
            barrelshifter: None,
            operand_count: 2,
        };

        set_register_value(&mut registers, "X0", RegisterValue::Val64(314));
        set_register_value(&mut registers, "X1", RegisterValue::Val64(512));
        let converted = convert_ins(&ins, &registers).expect("Conversion failed for no reason.");
        match converted {
            Instructions::Str { ref op1, op2 } => str(&mut registers, op1, op2, &mut memory)
                .expect("Instruction failed for no reason."),
            _ => panic!("convert_ins converted wrongly."),
        }

        let converted = convert_ins(&ins2, &registers).expect("Conversion failed for no reason.");
        match converted {
            Instructions::Ldr { ref op1, ref op2 } => {
                ldr(&mut registers, op1, op2, &memory).expect("Instruction failed for no reason.")
            }
            _ => panic!("convert_ins converted wrongly."),
        }

        let value = get_register_value(&registers, "X0").unwrap();

        assert_eq!(value, RegisterValue::Val64(314), "Value #1 didn't match.");

        set_register_value(&mut registers, "X1", RegisterValue::Val64(256));
        let converted = convert_ins(&ins, &registers).expect("Conversion failed for no reason.");
        match converted {
            Instructions::Str { ref op1, op2 } => str(&mut registers, op1, op2, &mut memory)
                .expect("Instruction failed for no reason."),
            _ => panic!("convert_ins converted wrongly."),
        }

        let converted = convert_ins(&ins2, &registers).expect("Conversion failed for no reason.");
        match converted {
            Instructions::Ldr { ref op1, ref op2 } => {
                ldr(&mut registers, op1, op2, &memory).expect("Instruction failed for no reason.")
            }
            _ => panic!("convert_ins converted wrongly."),
        }

        let value2 = get_register_value(&registers, "X0").unwrap();
        let value3 = get_register_value(&registers, "X1").unwrap();

        assert_eq!(value2, RegisterValue::Val64(314), "Value #2 didn't match.");
        assert_eq!(
            value3,
            RegisterValue::Val64(314),
            "Post indexing didn't work."
        );

        let converted = convert_ins(&ins3, &registers).expect("Conversion failed for no reason.");
        match converted {
            Instructions::Ldr { ref op1, ref op2 } => {
                ldr(&mut registers, op1, op2, &memory).expect("Instruction failed for no reason.")
            }
            _ => panic!("convert_ins converted wrongly."),
        }

        let value4 = get_register_value(&registers, "X0").unwrap();
        assert_eq!(value4, RegisterValue::Val64(271), "Value #3 didn't match.");
    }

    #[test]
    fn cmp_test() {
        let mut registers = create_registers();
        let ins = Instruction {
            name: String::from("CMP"),
            op1: Some(Operand::OperandRegister(String::from("X0"))),
            op2: Some(Operand::OperandRegister(String::from("X1"))),
            op3: None,
            op4: None,
            barrelshifter: None,
            operand_count: 2,
        };

        let converted = convert_ins(&ins, &registers).expect("Conversion failed for no reason.");

        set_register_value(&mut registers, "X0", RegisterValue::Val64(16));
        set_register_value(&mut registers, "X1", RegisterValue::Val64(16));

        match converted {
            Instructions::Cmp { ref op1, ref op2 } => cmp(&mut registers, op1, op2),
            _ => panic!("Conversion ran wrong."),
        }

        assert!(!get_flag(&registers, "N"), "CMP #1 N flag is set.");
        assert!(get_flag(&registers, "Z"), "CMP #1 Z flag is cleared.");
        assert!(get_flag(&registers, "C"), "CMP #1 C flag is cleared.");
        assert!(!get_flag(&registers, "V"), "CMP #1 V flag is set.");

        set_register_value(&mut registers, "X0", RegisterValue::Val64(17));
        set_register_value(&mut registers, "X1", RegisterValue::Val64(16));

        match converted {
            Instructions::Cmp { ref op1, ref op2 } => cmp(&mut registers, op1, op2),
            _ => panic!("Conversion ran wrong."),
        }

        assert!(!get_flag(&registers, "N"), "CMP #2 N flag is set.");
        assert!(!get_flag(&registers, "Z"), "CMP #2 Z flag is set.");
        assert!(get_flag(&registers, "C"), "CMP #2 C flag is cleared.");
        assert!(!get_flag(&registers, "V"), "CMP #2 V flag is set.");

        set_register_value(&mut registers, "X0", RegisterValue::Val64(16));
        set_register_value(&mut registers, "X1", RegisterValue::Val64(17));

        match converted {
            Instructions::Cmp { ref op1, ref op2 } => cmp(&mut registers, op1, op2),
            _ => panic!("Conversion ran wrong."),
        }

        assert!(get_flag(&registers, "N"), "CMP #3 N flag is cleared.");
        assert!(!get_flag(&registers, "Z"), "CMP #3 Z flag is set.");
        assert!(!get_flag(&registers, "C"), "CMP #3 C flag is set.");
        assert!(!get_flag(&registers, "V"), "CMP #3 V flag is set.");

        set_register_value(
            &mut registers,
            "X0",
            RegisterValue::Val64(9223372036854775809),
        );
        set_register_value(
            &mut registers,
            "X1",
            RegisterValue::Val64(9223372036854775809),
        );

        match converted {
            Instructions::Cmp { ref op1, ref op2 } => cmp(&mut registers, op1, op2),
            _ => panic!("Conversion ran wrong."),
        }

        assert!(!get_flag(&registers, "N"), "CMP #4 N flag is set.");
        assert!(get_flag(&registers, "Z"), "CMP #4 Z flag is cleared.");
        assert!(get_flag(&registers, "C"), "CMP #4 C flag is cleared.");
        assert!(get_flag(&registers, "V"), "CMP #4 V flag is cleared.");
    }

    #[test]
    fn b_test() {
        let mut registers = create_registers();

        b(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(10)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(6),
            "Branch #1 didn't work."
        );
        b(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(50)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(52),
            "Branch #2 didn't work."
        );
        b(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(4)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(52),
            "Branch #3 didn't work."
        );
        b(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(20)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(68),
            "Branch #4 didn't work."
        );
    }

    #[test]
    fn beq_test() {
        let mut registers = create_registers();

        beq(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(10)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(0),
            "Branch #1 didn't work."
        );

        set_flag(&mut registers, "Z", true);
        beq(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(50)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(46),
            "Branch #2 didn't work."
        );

        set_flag(&mut registers, "Z", false);
        beq(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(1)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(46),
            "Branch #3 didn't work."
        );

        set_flag(&mut registers, "Z", true);
        beq(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(20)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(62),
            "Branch #4 didn't work."
        );
    }

    #[test]
    fn bne_test() {
        let mut registers = create_registers();

        set_flag(&mut registers, "Z", true);
        bne(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(10)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(0),
            "Branch #1 didn't work."
        );

        set_flag(&mut registers, "Z", false);
        bne(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(50)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(46),
            "Branch #2 didn't work."
        );

        set_flag(&mut registers, "Z", true);
        bne(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(1)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(46),
            "Branch #3 didn't work."
        );

        set_flag(&mut registers, "Z", false);
        bne(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(20)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(62),
            "Branch #4 didn't work."
        );
    }

    #[test]
    fn bgt_test() {
        let mut registers = create_registers();

        set_flag(&mut registers, "Z", true);
        bgt(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(10)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(0),
            "Branch #1 didn't work."
        );

        set_flag(&mut registers, "Z", false);
        bgt(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(50)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(46),
            "Branch #2 didn't work."
        );

        set_flag(&mut registers, "Z", true);
        bgt(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(1)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(46),
            "Branch #3 didn't work."
        );

        set_flag(&mut registers, "Z", false);
        bgt(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(20)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(62),
            "Branch #4 didn't work."
        );
    }

    #[test]
    fn blt_test() {
        let mut registers = create_registers();

        set_flag(&mut registers, "N", false);
        blt(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(10)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(0),
            "Branch #1 didn't work."
        );

        set_flag(&mut registers, "N", true);
        blt(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(50)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(46),
            "Branch #2 didn't work."
        );

        set_flag(&mut registers, "N", false);
        blt(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(1)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(46),
            "Branch #3 didn't work."
        );

        set_flag(&mut registers, "N", true);
        blt(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(20)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(62),
            "Branch #4 didn't work."
        );
    }

    #[test]
    fn bge_test() {
        let mut registers = create_registers();

        set_flag(&mut registers, "N", true);
        bge(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(10)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(0),
            "Branch #1 didn't work."
        );

        set_flag(&mut registers, "N", false);
        bge(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(50)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(46),
            "Branch #2 didn't work."
        );

        set_flag(&mut registers, "N", true);
        bge(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(1)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(46),
            "Branch #3 didn't work."
        );

        set_flag(&mut registers, "N", false);
        bge(
            &mut registers,
            &Operand::OperandNumber(RegisterValue::Val64(20)),
        );
        assert_eq!(
            get_register_value(&registers, "PC").unwrap(),
            RegisterValue::Val64(62),
            "Branch #4 didn't work."
        );
    }

    #[test]
    fn adr_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "PC", RegisterValue::Val64(12));
        adr(
            &mut registers,
            "X0",
            &Operand::OperandNumber(RegisterValue::Val64(20)),
        );
        set_register_value(&mut registers, "PC", RegisterValue::Val64(20));
        adr(
            &mut registers,
            "X1",
            &Operand::OperandNumber(RegisterValue::Val64(20)),
        );
        set_register_value(&mut registers, "PC", RegisterValue::Val64(28));
        adr(
            &mut registers,
            "X2",
            &Operand::OperandNumber(RegisterValue::Val64(20)),
        );

        let reg_val1 = get_register_value(&registers, "X0").unwrap();
        let reg_val2 = get_register_value(&registers, "X1").unwrap();
        let reg_val3 = get_register_value(&registers, "X2").unwrap();

        assert_eq!(
            reg_val1,
            RegisterValue::Val64(32),
            "ADR #1 calculated the answer wrongly."
        );
        assert_eq!(
            reg_val2,
            RegisterValue::Val64(40),
            "ADR #2 calculated the answer wrongly."
        );
        assert_eq!(
            reg_val3,
            RegisterValue::Val64(48),
            "ADR #3 calculated the answer wrongly."
        );
    }

    #[test]
    fn movk_test() {
        let mut registers = create_registers();

        movk(
            &mut registers,
            &Instruction {
                name: "MOVK".to_string(),
                op1: Some(Operand::OperandRegister("X0".to_string())),
                op2: Some(Operand::OperandNumber(RegisterValue::Val64(1000))),
                op3: None,
                op4: None,
                barrelshifter: Some(BarrelShifter {
                    barrelshiftertype: BarrelShifterType::LSL,
                    value: Some(RegisterValue::Val64(0)),
                }),
                operand_count: 2,
            },
        );
        assert_eq!(
            get_register_value(&registers, "X0").unwrap().convert_64(),
            1000,
            "MOVK #1 didn't work."
        );
        movk(
            &mut registers,
            &Instruction {
                name: "MOVK".to_string(),
                op1: Some(Operand::OperandRegister("X0".to_string())),
                op2: Some(Operand::OperandNumber(RegisterValue::Val64(1000))),
                op3: None,
                op4: None,
                barrelshifter: Some(BarrelShifter {
                    barrelshiftertype: BarrelShifterType::LSL,
                    value: Some(RegisterValue::Val64(16)),
                }),
                operand_count: 2,
            },
        );
        assert_eq!(
            get_register_value(&registers, "X0").unwrap().convert_64(),
            65537000,
            "MOVK #2 didn't work."
        );
        movk(
            &mut registers,
            &Instruction {
                name: "MOVK".to_string(),
                op1: Some(Operand::OperandRegister("X0".to_string())),
                op2: Some(Operand::OperandNumber(RegisterValue::Val64(1000))),
                op3: None,
                op4: None,
                barrelshifter: Some(BarrelShifter {
                    barrelshiftertype: BarrelShifterType::LSL,
                    value: Some(RegisterValue::Val64(32)),
                }),
                operand_count: 2,
            },
        );
        assert_eq!(
            get_register_value(&registers, "X0").unwrap().convert_64(),
            4295032833000,
            "MOVK #3 didn't work."
        );
        movk(
            &mut registers,
            &Instruction {
                name: "MOVK".to_string(),
                op1: Some(Operand::OperandRegister("X0".to_string())),
                op2: Some(Operand::OperandNumber(RegisterValue::Val64(1000))),
                op3: None,
                op4: None,
                barrelshifter: Some(BarrelShifter {
                    barrelshiftertype: BarrelShifterType::LSL,
                    value: Some(RegisterValue::Val64(48)),
                }),
                operand_count: 2,
            },
        );
        assert_eq!(
            get_register_value(&registers, "X0").unwrap().convert_64(),
            281479271743489000,
            "MOVK #4 didn't work."
        );
        movk(
            &mut registers,
            &Instruction {
                name: "MOVK".to_string(),
                op1: Some(Operand::OperandRegister("W1".to_string())),
                op2: Some(Operand::OperandNumber(RegisterValue::Val64(1000))),
                op3: None,
                op4: None,
                barrelshifter: Some(BarrelShifter {
                    barrelshiftertype: BarrelShifterType::LSL,
                    value: Some(RegisterValue::Val64(16)),
                }),
                operand_count: 2,
            },
        );
        assert_eq!(
            get_register_value(&registers, "W1").unwrap().convert_32(),
            65536000,
            "MOVK #5 didn't work."
        );
    }

    #[test]
    fn parse_num_test() {
        let res1 = parse_num("10").expect("Parse num #1 failed for no reason.");
        let res2 = parse_num("18446744073709551616");
        let res3 = parse_num("-1").expect("Parse num #3 failed for no reason.");

        assert_eq!(res1, 10, "Parse num #1 parsed wrong.");
        assert!(matches!(res2, None), "Parse num #2 parsed wrong.");
        assert_eq!(res3, 18446744073709551615, "Parse num #3 parsed wrong.");
    }

    #[test]
    fn parse_num_hex_test() {
        let res1 = parse_num_hex("0x10").expect("Parse num #1 failed for no reason.");
        let res2 = parse_num_hex("18446744073709551616");
        let res3 = parse_num_hex("-0x1").expect("Parse num #3 failed for no reason.");
        let res4 = parse_num_hex("1").expect("Parse num #4 failed for no reason.");
        let res5 = parse_num_hex("-10").expect("Parse num #5 failed for no reason.");

        assert_eq!(res1, 0x10, "Parse num #1 parsed wrong.");
        assert!(matches!(res2, None), "Parse num #2 parsed wrong.");
        assert_eq!(res3, 0xffffffffffffffff, "Parse num #3 parsed wrong.");
        assert_eq!(res4, 0x1, "Parse num #4 parsed wrong.");
        assert_eq!(res5, 0xfffffffffffffff0, "Parse num #5 parsed wrong.");
    }

    #[test]
    fn adrp_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "PC", RegisterValue::Val64(0x4000c0));
        adrp(
            &mut registers,
            "X0",
            &Operand::OperandNumber(RegisterValue::Val64(0)),
        );

        assert_eq!(
            get_register_value(&registers, "X0").unwrap().convert_64(),
            0x400000,
            "ADRP worked wrongly."
        );
    }

    #[test]
    fn orr_test() {
        let mut registers = create_registers();

        set_register_value(
            &mut registers,
            "X1",
            RegisterValue::Val64(11134694135857075030),
        );
        set_register_value(
            &mut registers,
            "X2",
            RegisterValue::Val64(2353745857404082624),
        );
        orr(
            &mut registers,
            "X0",
            "X1",
            &Operand::OperandRegister("X2".to_string()),
        );

        assert_eq!(
            get_register_value(&registers, "X0").unwrap().convert_64(),
            13451811676621822934,
            "ORR worked wrongly."
        );
    }

    #[test]
    fn eor_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X1", RegisterValue::Val64(314));
        set_register_value(&mut registers, "X2", RegisterValue::Val64(271));
        eor(
            &mut registers,
            "X0",
            "X1",
            &Operand::OperandRegister("X2".to_string()),
        );

        assert_eq!(
            get_register_value(&registers, "X0").unwrap().convert_64(),
            53,
            "EOR worked wrongly."
        );
    }

    #[test]
    fn eon_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X1", RegisterValue::Val64(314));
        set_register_value(
            &mut registers,
            "X2",
            RegisterValue::Val64(0xfffffffffffffef0),
        );
        eon(
            &mut registers,
            "X0",
            "X1",
            &Operand::OperandRegister("X2".to_string()),
        );

        assert_eq!(
            get_register_value(&registers, "X0").unwrap().convert_64(),
            53,
            "EON worked wrongly."
        );
    }

    #[test]
    fn bic_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X1", RegisterValue::Val64(12));
        set_register_value(
            &mut registers,
            "X2",
            RegisterValue::Val64(0xffffffffffffffe7),
        );
        bic(
            &mut registers,
            "X0",
            "X1",
            &Operand::OperandRegister("X2".to_string()),
        );

        assert_eq!(
            get_register_value(&registers, "X0").unwrap().convert_64(),
            8,
            "BIC worked wrongly."
        );
    }

    #[test]
    fn lsl_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X1", RegisterValue::Val64(16));
        lsl(
            &mut registers,
            "X0",
            "X1",
            &Operand::OperandNumber(RegisterValue::Val64(1)),
        );

        assert_eq!(
            get_register_value(&registers, "X0").unwrap().convert_64(),
            32,
            "LSL worked wrongly."
        );
    }

    #[test]
    fn lsr_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X1", RegisterValue::Val64(64));
        lsr(
            &mut registers,
            "X0",
            "X1",
            &Operand::OperandNumber(RegisterValue::Val64(1)),
        );

        assert_eq!(
            get_register_value(&registers, "X0").unwrap().convert_64(),
            32,
            "LSR worked wrongly."
        );
    }

    #[test]
    fn asr_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X1", RegisterValue::Val64(64));
        set_register_value(&mut registers, "W3", RegisterValue::Val64(128));
        asr(
            &mut registers,
            "X0",
            "X1",
            &Operand::OperandNumber(RegisterValue::Val32(1)),
        );
        asr(
            &mut registers,
            "W2",
            "W3",
            &Operand::OperandNumber(RegisterValue::Val32(3)),
        );

        let output1 = get_register_value(&registers, "X0").unwrap();
        let output2 = get_register_value(&registers, "W2").unwrap();

        assert_eq!(output1, RegisterValue::Val64(32), "ASR #1 worked wrongly.");
        assert_eq!(output2, RegisterValue::Val32(16), "ASR #2 worked wrongly.");
    }

    #[test]
    fn ror_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X1", RegisterValue::Val64(628));
        ror(
            &mut registers,
            "X0",
            "X1",
            &Operand::OperandNumber(RegisterValue::Val64(1)),
        );

        assert_eq!(
            get_register_value(&registers, "X0").unwrap().convert_64(),
            314,
            "ROR worked wrongly."
        );
    }

    #[test]
    fn ubfx_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X1", RegisterValue::Val64(2882343476));
        ubfx(
            &mut registers,
            "X0",
            "X1",
            &Operand::OperandNumber(RegisterValue::Val64(4)),
            &Operand::OperandNumber(RegisterValue::Val64(12)),
        );

        assert_eq!(
            get_register_value(&registers, "X0").unwrap().convert_64(),
            291,
            "ROR worked wrongly."
        );
    }

    #[test]
    fn sbfx_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "W1", RegisterValue::Val64(62976));
        sbfx(
            &mut registers,
            "W0",
            "W1",
            &Operand::OperandNumber(RegisterValue::Val64(8)),
            &Operand::OperandNumber(RegisterValue::Val64(8)),
        );

        assert_eq!(
            get_register_value(&registers, "W0").unwrap().convert_32(),
            4294967286,
            "ROR worked wrongly."
        );
    }

    #[test]
    fn adc_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X2", RegisterValue::Val64(10));
        set_register_value(&mut registers, "X3", RegisterValue::Val64(20));
        adc(&mut registers, "X0", "X2", "X3");
        set_flag(&mut registers, "C", true);
        adc(&mut registers, "X1", "X2", "X3");

        let output1 = get_register_value(&registers, "X0").unwrap().convert_64();
        let output2 = get_register_value(&registers, "X1").unwrap().convert_64();

        assert_eq!(output1, 30, "ADC #1 worked wrongly.");
        assert_eq!(output2, 31, "ADC #2 worked wrongly.");
    }

    #[test]
    fn sbc_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X2", RegisterValue::Val64(40));
        set_register_value(&mut registers, "X3", RegisterValue::Val64(10));
        sbc(&mut registers, "X0", "X2", "X3");
        set_flag(&mut registers, "C", true);
        sbc(&mut registers, "X1", "X2", "X3");

        let output1 = get_register_value(&registers, "X0").unwrap().convert_64();
        let output2 = get_register_value(&registers, "X1").unwrap().convert_64();

        assert_eq!(output1, 29, "SBC #1 worked wrongly.");
        assert_eq!(output2, 30, "SBC #2 worked wrongly.");
    }

    #[test]
    fn neg_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X2", RegisterValue::Val64(255));
        set_register_value(&mut registers, "W3", RegisterValue::Val32(255));
        neg(&mut registers, "X0", "X2");
        neg(&mut registers, "W1", "W3");

        let output1 = get_register_value(&registers, "X0").unwrap().convert_64();
        let output2 = get_register_value(&registers, "W1").unwrap().convert_32();

        assert_eq!(output1, 0xffffffffffffff01, "NEG #1 worked wrongly.");
        assert_eq!(output2, 0xffffff01, "NEG #2 worked wrongly.");
    }

    #[test]
    fn negs_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X6", RegisterValue::Val64(0));
        set_register_value(&mut registers, "W7", RegisterValue::Val32(0));
        set_register_value(&mut registers, "X8", RegisterValue::Val64(1));
        set_register_value(&mut registers, "W9", RegisterValue::Val32(1));
        set_register_value(
            &mut registers,
            "X10",
            RegisterValue::Val64(0x8000000000000000),
        );
        set_register_value(&mut registers, "W11", RegisterValue::Val32(0x80000000));
        negs(&mut registers, "X0", "X6");
        assert!(!get_flag(&registers, "N"), "NEGS #1 N flag is set.");
        assert!(get_flag(&registers, "Z"), "NEGS #1 Z flag isn't set.");
        assert!(get_flag(&registers, "C"), "NEGS #1 C flag isn't set.");
        assert!(!get_flag(&registers, "V"), "NEGS #1 V flag is set.");
        negs(&mut registers, "W1", "W7");
        assert!(!get_flag(&registers, "N"), "NEGS #2 N flag is set.");
        assert!(get_flag(&registers, "Z"), "NEGS #2 Z flag isn't set.");
        assert!(get_flag(&registers, "C"), "NEGS #2 C flag isn't set.");
        assert!(!get_flag(&registers, "V"), "NEGS #2 V flag is set.");
        negs(&mut registers, "X2", "X8");
        assert!(get_flag(&registers, "N"), "NEGS #3 N flag isn't set.");
        assert!(!get_flag(&registers, "Z"), "NEGS #3 Z flag is set.");
        assert!(!get_flag(&registers, "C"), "NEGS #3 C flag is set.");
        assert!(!get_flag(&registers, "V"), "NEGS #3 V flag is set.");
        negs(&mut registers, "W3", "W9");
        assert!(get_flag(&registers, "N"), "NEGS #4 N flag isn't set.");
        assert!(!get_flag(&registers, "Z"), "NEGS #4 Z flag is set.");
        assert!(!get_flag(&registers, "C"), "NEGS #4 C flag is set.");
        assert!(!get_flag(&registers, "V"), "NEGS #4 V flag is set.");
        negs(&mut registers, "X4", "X10");
        assert!(get_flag(&registers, "N"), "NEGS #5 N flag isn't set.");
        assert!(!get_flag(&registers, "Z"), "NEGS #5 Z flag is set.");
        assert!(!get_flag(&registers, "C"), "NEGS #5 C flag is set.");
        assert!(get_flag(&registers, "V"), "NEGS #5 V flag isn't set.");
        negs(&mut registers, "W5", "W11");
        assert!(get_flag(&registers, "N"), "NEGS #6 N flag isn't set.");
        assert!(!get_flag(&registers, "Z"), "NEGS #6 Z flag is set.");
        assert!(!get_flag(&registers, "C"), "NEGS #6 C flag is set.");
        assert!(get_flag(&registers, "V"), "NEGS #6 V flag isn't set.");
    }

    #[test]
    fn ldrb_test() {
        let mut registers = create_registers();
        let memory = vec![5, 0, 4, 2, 0, 2, 3, 3, 9, 0, 0, 0, 1, 0, 4, 1];

        set_register_value(&mut registers, "X1", RegisterValue::Val64(8));
        ldrb(
            &mut registers,
            "X0",
            MemoryAddress {
                base_address: String::from("X1"),
                second_val: None,
                barrelshifter: None,
                postindexval: None,
                addr_type: MemoryAddressType::Normal,
            },
            &memory,
        )
        .expect("LDRB #1 failed for no reason.");
        set_register_value(&mut registers, "X1", RegisterValue::Val64(9999));
        let output2 = ldrb(
            &mut registers,
            "X0",
            MemoryAddress {
                base_address: String::from("X1"),
                second_val: None,
                barrelshifter: None,
                postindexval: None,
                addr_type: MemoryAddressType::Normal,
            },
            &memory,
        );

        assert_eq!(
            get_register_value(&registers, "X0").unwrap(),
            RegisterValue::Val64(9),
            "LDRB #1 worked wrongly."
        );
        assert_eq!(
            output2,
            Err(String::from("Invalid memory address")),
            "LDRB #2 worked wrongly or threw wrong error."
        );
    }

    #[test]
    fn ldrh_test() {
        let mut registers = create_registers();
        let memory = vec![0x0a, 0x0b];

        set_register_value(&mut registers, "X1", RegisterValue::Val64(0));
        let _ = ldrh(
            &mut registers,
            "X0",
            MemoryAddress {
                base_address: String::from("X1"),
                second_val: None,
                barrelshifter: None,
                postindexval: None,
                addr_type: MemoryAddressType::Normal,
            },
            &memory,
        );

        let output = get_register_value(&registers, "X0").unwrap().convert_64();

        assert_eq!(output, 2826, "LDRH didn't work correctly.");
    }

    #[test]
    fn ldrsw_test() {
        let mut registers = create_registers();
        let memory = vec![0x00, 0x00, 0xff, 0xff, 0x00, 0x01];

        set_register_value(&mut registers, "X2", RegisterValue::Val64(0));
        set_register_value(&mut registers, "X3", RegisterValue::Val64(2));
        let _ = ldrsw(
            &mut registers,
            "X0",
            MemoryAddress {
                base_address: String::from("X2"),
                second_val: None,
                barrelshifter: None,
                postindexval: None,
                addr_type: MemoryAddressType::Normal,
            },
            &memory,
        )
        .expect("LDRSW #1 failed for no reason");
        let _ = ldrsw(
            &mut registers,
            "X1",
            MemoryAddress {
                base_address: String::from("X3"),
                second_val: None,
                barrelshifter: None,
                postindexval: None,
                addr_type: MemoryAddressType::Normal,
            },
            &memory,
        )
        .expect("LDRSW #2 failed for no reason");

        let output1 = get_register_value(&registers, "X0").unwrap();
        let output2 = get_register_value(&registers, "X1").unwrap();

        assert_eq!(
            output1,
            RegisterValue::Val64(0xffffffffffff0000),
            "LDRSW #1 ran wrongly."
        );
        assert_eq!(
            output2,
            RegisterValue::Val64(0x0100ffff),
            "LDRSW #1 ran wrongly."
        );
    }

    #[test]
    fn strb_test() {
        let mut registers = create_registers();
        let mut memory = vec![0; 16];

        set_register_value(&mut registers, "X0", RegisterValue::Val64(1));
        set_register_value(&mut registers, "X1", RegisterValue::Val64(0));
        let _ = strb(
            &mut registers,
            "X0",
            MemoryAddress {
                base_address: String::from("X1"),
                second_val: None,
                barrelshifter: None,
                postindexval: None,
                addr_type: MemoryAddressType::Normal,
            },
            &mut memory,
        )
        .expect("STRB failed for no reason.");

        assert_eq!(memory[0], 1, "STRB didn't work correctly.");
    }

    #[test]
    fn strh_test() {
        let mut registers = create_registers();
        let mut memory = vec![0; 16];

        set_register_value(&mut registers, "X0", RegisterValue::Val64(31415));
        set_register_value(&mut registers, "X1", RegisterValue::Val64(0));
        let _ = strh(
            &mut registers,
            "X0",
            MemoryAddress {
                base_address: String::from("X1"),
                second_val: None,
                barrelshifter: None,
                postindexval: None,
                addr_type: MemoryAddressType::Normal,
            },
            &mut memory,
        )
        .expect("STRH failed for no reason.");

        assert_eq!(memory[0], 183, "STRH didn't assign byte #1 correctly.");
        assert_eq!(memory[1], 122, "STRH didn't assign byte #2 correctly.");
    }

    #[test]
    fn ldp_test() {
        let mut registers = create_registers();
        let memory = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        set_register_value(&mut registers, "X2", RegisterValue::Val64(0));
        let _ = ldp(
            &mut registers,
            "X0",
            "X1",
            MemoryAddress {
                base_address: String::from("X2"),
                second_val: None,
                barrelshifter: None,
                postindexval: None,
                addr_type: MemoryAddressType::Normal,
            },
            &memory,
        )
        .expect("LDP failed for no reason");

        let output1 = get_register_value(&registers, "X0").unwrap().convert_64();
        let output2 = get_register_value(&registers, "X1").unwrap().convert_64();

        assert_eq!(output1, 0x0807060504030201, "LDP didn't load X0 correctly.");
        assert_eq!(output2, 0x100f0e0d0c0b0a09, "LDP didn't load X1 correctly.");
    }

    #[test]
    fn stp_test() {
        let mut registers = create_registers();
        let mut memory = vec![0; 16];

        set_register_value(
            &mut registers,
            "X0",
            RegisterValue::Val64(0x0807060504030201),
        );
        set_register_value(
            &mut registers,
            "X1",
            RegisterValue::Val64(0x100f0e0d0c0b0a09),
        );
        set_register_value(&mut registers, "X2", RegisterValue::Val64(0));
        let _ = stp(
            &mut registers,
            "X0",
            "X1",
            MemoryAddress {
                base_address: String::from("X2"),
                second_val: None,
                barrelshifter: None,
                postindexval: None,
                addr_type: MemoryAddressType::Normal,
            },
            &mut memory,
        )
        .expect("STP failed for no reason");

        for i in 1..17 {
            assert_eq!(memory[i - 1], i as u8, "STP wrote byte #{} wrongly.", i);
        }
    }

    #[test]
    fn cbz_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X0", RegisterValue::Val64(0));
        set_register_value(&mut registers, "X1", RegisterValue::Val64(1));
        cbz(
            &mut registers,
            "X0",
            &Operand::OperandNumber(RegisterValue::Val64(10)),
        );
        let output1 = get_register_value(&registers, "PC").unwrap().convert_64();
        cbz(
            &mut registers,
            "X1",
            &Operand::OperandNumber(RegisterValue::Val64(10)),
        );
        let output2 = get_register_value(&registers, "PC").unwrap().convert_64();

        assert_eq!(output1, 6, "CBZ #1 didn't branch.");
        assert_eq!(output2, 6, "CBZ #1 branched.");
    }

    #[test]
    fn cbnz_test() {
        let mut registers = create_registers();

        set_register_value(&mut registers, "X0", RegisterValue::Val64(0));
        set_register_value(&mut registers, "X1", RegisterValue::Val64(1));
        cbnz(
            &mut registers,
            "X0",
            &Operand::OperandNumber(RegisterValue::Val64(10)),
        );
        let output1 = get_register_value(&registers, "PC").unwrap().convert_64();
        cbnz(
            &mut registers,
            "X1",
            &Operand::OperandNumber(RegisterValue::Val64(10)),
        );
        let output2 = get_register_value(&registers, "PC").unwrap().convert_64();

        assert_eq!(output1, 0, "CBNZ #1 branched.");
        assert_eq!(output2, 6, "CBNZ #1 didn't branch.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers::*;
    use crate::*;

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

        assert_eq!(registers.len(), 64, "Not enough registers was created.");
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
            } else {
                assert_eq!(registers[i].name, "PC", "Register name didn't match.");
            }

            if i > 30 && i < 62 {
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
        assert_eq!(registers[0].value, RegisterValue::Val64(1));
        assert_eq!(registers[31].value, RegisterValue::Val32(1));
        assert_eq!(registers[1].value, RegisterValue::Val64(2));
        assert_eq!(registers[32].value, RegisterValue::Val32(2));
    }
}

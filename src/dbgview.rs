use crate::{errormsg::*, instruction_parser::*, registers::*};
use std::io::{self, Write};
use std::process::exit;

fn clear_screen() {
    println!("\x1bc");
}

fn print_msg(msg: &str) {
    let (w, _) = match termion::terminal_size() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("Can not get terminal size because of '{}'", e);
            exit(1);
        }
    };
    let msg_len = msg.len();
    let line = "─".repeat(((w as usize) - msg_len) / 2);
    print!("\x1b");
    if ((w as usize) - msg_len) % 2 != 0 {
        print!("─");
    }

    println!("{}", format!("[93m{}{}{}\x1b[0m", line, msg, line));
}

fn add_space(reg_name: &str) -> String {
    if reg_name.len() == 2 {
        return format!("  {}", reg_name);
    } else if reg_name.len() == 3 {
        return format!(" {}", reg_name);
    }

    reg_name.to_string()
}

pub fn debug_view(
    registers: &mut Vec<Register>,
    instructions: &Vec<Instruction>,
    last_msg: &str,
) -> String {
    let pc = match get_register_value(registers, "PC") {
        Some(RegisterValue::Val64(n)) => n,
        _ => unreachable!(),
    };
    clear_screen();
    print_msg(" - INSTRUCTIONS - ");
    let ins1 = match instructions.get(pc as usize) {
        Some(n) => n,
        None => return String::new(),
    };
    let ins2 = instructions.get((pc + 1) as usize);
    let ins3 = instructions.get((pc + 2) as usize);
    let mut i = 0;
    let mut input;
    let mut ins_to_print = vec![ins1];
    let mut trimmed_input;
    let mut parts: Vec<&str>;

    if ins2.is_some() {
        ins_to_print.push(ins2.unwrap());
    }

    if ins3.is_some() {
        ins_to_print.push(ins3.unwrap());
    }

    for ins in &ins_to_print {
        print!("{:#X}: \x1b[31m{}\x1b[0m", pc + i, ins.name);
        match &ins.op1 {
            Some(n) => print!(" {}", n),
            None => (),
        }

        match &ins.op2 {
            Some(n) => print!(", {}", n),
            None => (),
        }

        match &ins.op3 {
            Some(n) => print!(", {}", n),
            None => (),
        }

        match &ins.op4 {
            Some(n) => print!(", {}", n),
            None => (),
        }

        match &ins.barrelshifter {
            Some(n) => {
                print!(
                    ", {}",
                    match n.barrelshiftertype {
                        BarrelShifterType::LSL => "LSL",
                        BarrelShifterType::LSR => "LSR",
                        BarrelShifterType::ASR => "ASR",
                        BarrelShifterType::ROR => "ROR",
                        BarrelShifterType::RRX => "RRX",
                    }
                );
                match n.value {
                    Some(r) => match r {
                        RegisterValue::Val64(v) => print!(" {}", v),
                        RegisterValue::Val32(v) => print!(" {}", v),
                    },
                    None => (),
                }
            }
            None => (),
        }
        println!();
        i += 1;
    }
    print_msg(" - REGISTERS - ");
    for i in 0..15 {
        print!("\x1b[94m{}\x1b[0m: {:#X}\t\t\x1b[94m{}\x1b[0m: {:#X}\t\t\x1b[94m{}\x1b[0m: {:#X}\t\t\x1b[94m{}\x1b[0m: {:#X}\t\t", add_space(&registers[i].name), registers[i].value, add_space(&registers[i + 15].name), registers[i + 15].value, add_space(&registers[i + 30].name), registers[i + 30].value, add_space(&registers[i + 45].name), registers[i + 45].value);
        if i + 60 < registers.len() {
            print!(
                "\x1b[94m{}\x1b[0m: {:#X}",
                add_space(&registers[i + 60].name),
                registers[i + 60].value
            );
            if registers[i + 60].name == "NZCV" {
                print!(
                    " [ \x1b[{}mN\x1b[0m \x1b[{}mZ\x1b[0m \x1b[{}mC\x1b[0m \x1b[{}mV\x1b[0m ]",
                    match get_flag(registers, "N") {
                        true => 92,
                        false => 91,
                    },
                    match get_flag(registers, "Z") {
                        true => 92,
                        false => 91,
                    },
                    match get_flag(registers, "C") {
                        true => 92,
                        false => 91,
                    },
                    match get_flag(registers, "V") {
                        true => 92,
                        false => 91,
                    }
                );
            }
        }
        println!();
    }

    print_msg("");
    println!();
    loop {
        print!("\x1b[92m>\x1b[0m ");
        io::stdout().flush().unwrap();
        input = String::new();
        io::stdin().read_line(&mut input).unwrap_or_else(|_| {
            fail_normal("Unable to get input");
            exit(1);
        });

        if input.trim() == "" {
            input = last_msg.to_string();
        }

        trimmed_input = input.trim();
        parts = trimmed_input.split(" ").collect();
        if trimmed_input == "n" {
            break;
        } else if trimmed_input == "q" {
            exit(0);
        } else if trimmed_input == "help" {
            println!("Commands:");
            println!("\tn\t\t\tContinues to next instruction");
            println!("\tq\t\t\tExits the program");
            println!("\tset\t\t\tChanges something depending on the arguments");
            println!("\thelp\t\t\tShows this help message");
        } else if trimmed_input.starts_with("set") {
            if parts.len() == 4 && parts[1] == "reg" {
                if let Some(_) = get_register_value(registers, parts[2]) {
                    i = match parts[2].chars().nth(0).unwrap() {
                        'W' => 0,
                        _ => 1,
                    };

                    if i == 1 {
                        set_register_value(
                            registers,
                            parts[2],
                            RegisterValue::Val64(match parts[3].parse::<u64>() {
                                Ok(n) => n,
                                Err(_) => {
                                    fail_normal("Not valid number");
                                    continue;
                                }
                            }),
                        );
                    } else {
                        set_register_value(
                            registers,
                            parts[2],
                            RegisterValue::Val32(match parts[3].parse::<u32>() {
                                Ok(n) => n,
                                Err(_) => {
                                    fail_normal("Not valid number");
                                    continue;
                                }
                            }),
                        );
                    }
                } else {
                    fail_normal(&format!("No register found with name '{}'", parts[2]));
                }
            }
        } else {
            fail_normal(&format!("No command named '{}'", input.trim()));
        }
    }

    input
}

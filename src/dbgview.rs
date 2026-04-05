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
    memory: &Vec<u8>,
    addr: usize,
) -> String {
    let pc = match get_register_value(registers, "PC") {
        Some(RegisterValue::Val64(n)) => n,
        _ => unreachable!(),
    };
    clear_screen();
    print_msg(" - INSTRUCTIONS - ");
    let ins1 = match instructions.get((pc as usize) - addr) {
        Some(n) => n,
        None => return String::new(),
    };
    let ins2 = instructions.get(((pc + 1) as usize) - addr);
    let ins3 = instructions.get(((pc + 2) as usize) - addr);
    let mut i = 0;
    let mut input;
    let mut ins_to_print = vec![ins1];
    let mut trimmed_input;
    let mut parts: Vec<&str>;
    let mut address;
    let mut byte_count;
    let mut j;
    let mut k;

    if ins2.is_some() {
        ins_to_print.push(ins2.unwrap());
    }

    if ins3.is_some() {
        ins_to_print.push(ins3.unwrap());
    }

    for ins in &ins_to_print {
        print!("{:#X}: \x1b[31m{}\x1b[0m", pc + i, ins.name.to_uppercase());
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
                print!(", {}", n,);
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
            println!("\tv\t\t\tShows a memory region");
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
        } else if parts[0] == "v" {
            if parts.len() >= 2 {
                address = match parts[1].parse::<u64>() {
                    Ok(n) => n,
                    Err(_) => match u64::from_str_radix(&parts[1].replace("0x", ""), 16) {
                        Ok(n) => n,
                        Err(_) => {
                            fail_normal("Invalid address");
                            exit(1);
                        }
                    },
                };

                byte_count = match parts.get(2) {
                    Some(n) => n.parse::<u64>().unwrap_or_else(|_| {
                        fail_normal("Invalid byte count");
                        exit(1);
                    }),
                    None => 48,
                };

                j = 0;
                k = 0;
                i = 0;

                loop {
                    if i == 12 || j == byte_count {
                        if i != 12 {
                            print!("{}", "   ".repeat((12 - i) as usize));
                        }

                        print!("\x1b[90m|");
                        for i in 0..12 {
                            if memory[(address + k * 12 + i) as usize] < 32
                                || memory[(address + k * 12 + i) as usize] > 126
                            {
                                print!(".");
                            } else if memory[(address + k * 12 + i) as usize] == 46 {
                                print!("\x1b[91m.\x1b[0m");
                            } else {
                                print!("{}", memory[(address + k * 12 + i) as usize] as char);
                            }
                        }
                        k += 1;
                        i = 0;
                        println!("|\x1b[0m");
                    }

                    if j >= byte_count {
                        break;
                    }

                    if i == 0 {
                        print!("\x1b[1m\x1b[93m|{:#08X}|\x1b[0m ", address + k * 12);
                    }

                    print!(
                        "\x1b[1m\x1b[94m{:02X}\x1b[0m ",
                        memory[(address + k * 12 + i) as usize]
                    );
                    j += 1;
                    i += 1;
                }
                println!();
            } else {
                fail_normal("Not enough arguments for command view");
            }
        } else {
            fail_normal(&format!("No command named '{}'", input.trim()));
        }
    }

    input
}

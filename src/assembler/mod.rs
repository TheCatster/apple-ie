use super::cpu::operations::{get_instruction, AddressingMode};
use anyhow::Result;
use log::info;

pub fn assemble(program: &str) -> Result<Vec<u8>> {
    let lines = program.lines().map(|x| x.trim()).collect::<Vec<&str>>();
    let mut bytes = vec![];

    for line in lines {
        let line = line.trim();
        if !line.is_empty() {
            let (operation, mut args) = parse(line)?;
            bytes.push(operation);
            bytes.append(&mut args);
        }
    }

    info!("The following is in bytes: {:?}", bytes);

    Ok(bytes)
}

fn parse(line: &str) -> Result<(u8, Vec<u8>)> {
    let split: Vec<&str> = line.splitn(2, ' ').collect();

    let mut args = vec![];
    let mut addressing = AddressingMode::Implied;
    let operation_name = split[0];

    if split.len() != 1 {
        match split[1] {
            arg if arg.starts_with("#$") => {
                addressing = AddressingMode::Immediate;
                let arg = arg.trim_start_matches("#$");
                args.push(u8::from_str_radix(arg, 16).unwrap());
            }
            arg if arg.starts_with('$') => {
                let arg = arg.trim_start_matches('$');

                // if arg.len() == 2 - we use zeropage addressing
                match arg.len() {
                    2 => {
                        addressing = AddressingMode::ZeroPage;
                        args.push(u8::from_str_radix(arg, 16).unwrap());
                    }
                    4 => {
                        addressing = AddressingMode::Absolute;
                        let arg = u16::from_str_radix(arg, 16).unwrap();
                        args.push(arg as u8);
                        args.push((arg >> 8) as u8);
                    }
                    _ => panic!("Can't parse argument"),
                }
            }
            _ => panic!("unknown operand"),
        };
    }

    // first it looks only by name, if there are many opcodes with such name,
    // it looks by name and addressing
    // todo: change
    let opcode = get_instruction(None, Some(operation_name), Some(addressing))?;

    Ok((opcode.opcode as u8, args))
}

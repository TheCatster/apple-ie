use super::cpu::operations::{get_instruction, AddressingMode};
use anyhow::{bail, Result};
use log::info;

/// Assemble a program string into a vector of bytes.
///
/// # Arguments
///
/// * `program` - A string containing the program to assemble.
///
/// # Returns
///
/// A `Result` containing a vector of bytes representing the assembled program.
///
/// # Examples
///
/// ```
/// let program = "LDA #$01\nBRK";
/// let bytes = assembler::assemble(program).unwrap();
/// assert_eq!(bytes, vec![0xA9, 0x01, 0x00]);
/// ```
pub fn assemble(program: &str) -> Result<Vec<u8>> {
    // Split the program string into lines and trim whitespace
    let lines = program.lines().map(|x| x.trim()).collect::<Vec<&str>>();
    let mut bytes = vec![];

    // Iterate over each line of the program
    for line in lines {
        let line = line.trim();
        if !line.is_empty() {
            // Parse the line into an operation and its arguments
            let (operation, mut args) = parse(line)?;
            bytes.push(operation);
            bytes.append(&mut args);
        }
    }

    // Log the assembled program as a vector of bytes
    info!("The following is in bytes: {:#04X?}", bytes);

    Ok(bytes)
}

/// Parse a line of assembly code into an operation and its arguments.
///
/// # Arguments
///
/// * `line` - A string containing the line of assembly code to parse.
///
/// # Returns
///
/// A `Result` containing a tuple of the operation code and its arguments.
///
/// # Examples
///
/// ```
/// let line = "LDA #$01";
/// let (operation, args) = assembler::parse(line).unwrap();
/// assert_eq!(operation, 0xA9);
/// assert_eq!(args, vec![0x01]);
/// ```
fn parse(line: &str) -> Result<(u8, Vec<u8>)> {
    // Split the line into the operation name and its arguments
    let split: Vec<&str> = line.splitn(2, ' ').collect();

    let mut args = vec![];
    let mut addressing = AddressingMode::Implied;
    let operation_name = split[0];

    // If the line has arguments, parse them based on their addressing mode
    if split.len() != 1 {
        match split[1] {
            arg if arg.starts_with("#$") => {
                addressing = AddressingMode::Immediate;
                let arg = arg.trim_start_matches("#$");
                args.push(u8::from_str_radix(arg, 16)?);
            }
            arg if arg.starts_with('$') => {
                let arg = arg.trim_start_matches('$');

                // If the argument is two characters, use zero page addressing
                // If the argument is four characters, use absolute addressing
                match arg.len() {
                    2 => {
                        addressing = AddressingMode::ZeroPage;
                        args.push(u8::from_str_radix(arg, 16)?);
                    }
                    4 => {
                        addressing = AddressingMode::Absolute;
                        let arg = u16::from_str_radix(arg, 16)?;
                        args.push(arg as u8);
                        args.push((arg >> 8) as u8);
                    }
                    _ => bail!("Can't parse argument"),
                }
            }
            _ => bail!("Unknown operand."),
        };
    }

    // Look up the instruction based on the operation name and addressing mode
    match get_instruction(None, Some(operation_name), Some(addressing)) {
        Some(opcode) => Ok((opcode.opcode_value, args)),
        None => bail!("Instruction cannot be found."),
    }
}
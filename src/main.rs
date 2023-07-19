use assembler::{
    AddressingMode::{Absolute, ZeroPage},
    InstructionInfo,
    Opcode::{ADC, STA},
    OPCODE_SIZE_2, OPCODE_SIZE_3,
};
use cpu::CPU;

use clap::Parser;

mod assembler;
mod cpu;
mod memory;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// Specify an assembly file to run
    #[arg(short, long)]
    file: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut cpu: CPU = CPU::new();

    if cli.file.is_some() {
        let program = "LDA #$c0
                       TAX
                       INX
                       ADC #$c4";

        let bytes = assembler::assemble(&program);
        cpu.load(&bytes, 0x800);
    }

    cpu.run();
}

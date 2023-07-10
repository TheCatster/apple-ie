use assembler::{
    AddressingMode::{Absolute, ZeroPage},
    InstructionInfo,
    Opcode::{ADC, STA},
    OPCODE_SIZE_2, OPCODE_SIZE_3,
};
use cpu::CPUState;

mod assembler;
mod cpu;
mod memory;

fn main() {
    let mut cpu = CPUState::new();
    let instruction_set = vec![
        InstructionInfo {
            opcode: ADC,
            size: OPCODE_SIZE_2,
            addressing_mode: Absolute,
            cycle_count: 4,
        },
        InstructionInfo {
            opcode: STA,
            size: OPCODE_SIZE_3,
            addressing_mode: ZeroPage,
            cycle_count: 3,
        },
    ];

    cpu.begin(&instruction_set);
}

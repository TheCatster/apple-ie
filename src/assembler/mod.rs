pub const CPU_CLOCK_RATE: u32 = 1_000_000; // 1 MHz
pub const OPCODE_SIZE_1: usize = 1;
pub const OPCODE_SIZE_2: usize = 2;
pub const OPCODE_SIZE_3: usize = 3;
pub const DEFAULT_FLAGS: u8 = 0b0011_0000;

pub enum AddressingMode {
    Immediate,
    Absolute,
    ZeroPage,
}

pub enum Opcode {
    ADC, // Add with Carry
    STA, // Store Accumulator
    JMP, // Jump
}

pub struct InstructionInfo {
    pub opcode: Opcode,
    pub size: usize, // 1, 2 or 3 bytes
    pub addressing_mode: AddressingMode,
    pub cycle_count: usize,
}

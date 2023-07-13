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
    ADC,
    AND,
    ASL,
    STA,
    //...
}

pub struct InstructionInfo {
    pub opcode: Opcode,
    pub size: usize, // 1, 2 or 3 bytes
    pub addressing_mode: AddressingMode,
    pub cycle_count: usize,
}

pub fn decode(opcode: u8) -> InstructionInfo {
    match opcode {
        0x69 => InstructionInfo {
            opcode: Opcode::ADC,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Immediate,
            cycle_count: 2,
        },

        0x65 => InstructionInfo {
            opcode: Opcode::ADC,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
            cycle_count: 3,
        },

        // ... add other opcodes
        _ => panic!("Invalid opcode: {}", opcode),
    }
}

// ADC - Add with Carry
pub fn adc() {
    // implementation
}

// AND - Bitwise AND with Accumulator
pub fn and() {
    // implementation
}

// ASL - Arithmetic Shift Left
pub fn asl() {
    // implementation
}

// BCC - Branch on Carry Clear
pub fn bcc() {
    // implementation
}

// BCS - Branch on Carry Set
pub fn bcs() {
    // implementation
}

// BEQ - Branch on Equal
pub fn beq() {
    // implementation
}

// BIT - Test Bits
pub fn bit() {
    // implementation
}

// BMI - Branch on Minus
pub fn bmi() {
    // implementation
}

// BNE - Branch on Not Equal
pub fn bne() {
    // implementation
}

// BPL - Branch on Plus
pub fn bpl() {
    // implementation
}

// BRK - Break
pub fn brk() {
    // implementation
}

// BVC - Branch on Overflow Clear
pub fn bvc() {
    // implementation
}

// BVS - Branch on Overflow Set
pub fn bvs() {
    // implementation
}

// CLC - Clear Carry Flag
pub fn clc() {
    // implementation
}

// CLD - Clear Decimal Mode Flag
pub fn cld() {
    // implementation
}

// CLI - Clear Interrupt Disable Flag
pub fn cli() {
    // implementation
}

// CLV - Clear Overflow Flag
pub fn clv() {
    // implementation
}

// CMP - Compare Accumulator
pub fn cmp() {
    // implementation
}

// CPX - Compare X Register
pub fn cpx() {
    // implementation
}

// CPY - Compare Y Register
pub fn cpy() {
    // implementation
}

// DEC - Decrement Memory
pub fn dec() {
    // implementation
}

// DEX - Decrement X Register
pub fn dex() {
    // implementation
}

// DEY - Decrement Y Register
pub fn dey() {
    // implementation
}

// EOR - Exclusive OR
pub fn eor() {
    // implementation
}

// INC - Increment Memory
pub fn inc() {
    // implementation
}

// INX - Increment X Register
pub fn inx() {
    // implementation
}

// INY - Increment Y Register
pub fn iny() {
    // implementation
}

// JMP - Jump
pub fn jmp() {
    // implementation
}

// JSR - Jump to Subroutine
pub fn jsr() {
    // implementation
}

// LDA - Load Accumulator
pub fn lda() {
    // implementation
}

// LDX - Load X Register
pub fn ldx() {
    // implementation
}

// LDY - Load Y Register
pub fn ldy() {
    // implementation
}

// LSR - Logical Shift Right
pub fn lsr() {
    // implementation
}

// NOP - No Operation
pub fn nop() {
    // implementation
}

// ORA - Logical Inclusive OR
pub fn ora() {
    // implementation
}

// PHA - Push Accumulator
pub fn pha() {
    // implementation
}

// PHP - Push Processor Status
pub fn php() {
    // implementation
}

// PLA - Pull Accumulator
pub fn pla() {
    // implementation
}

// PLP - Pull Processor Status
pub fn plp() {
    // implementation
}

// ROL - Rotate Left
pub fn rol() {
    // implementation
}

// ROR - Rotate Right
pub fn ror() {
    // implementation
}

// RTI - Return from Interrupt
pub fn rti() {
    // implementation
}

// RTS - Return from Subroutine
pub fn rts() {
    // implementation
}

// SBC - Subtract with Carry
pub fn sbc() {
    // implementation
}

// SEC - Set Carry Flag
pub fn sec() {
    // implementation
}

// SED - Set Decimal Flag
pub fn sed() {
    // implementation
}

// SEI - Set Interrupt Disable Status
pub fn sei() {
    // implementation
}

// STA - Store Accumulator
pub fn sta() {
    // implementation
}

// STX - Store X Register
pub fn stx() {
    // implementation
}

// STY - Store Y Register
pub fn sty() {
    // implementation
}

// TAX - Transfer Accumulator to X
pub fn tax() {
    // implementation
}

// TAY - Transfer Accumulator to Y
pub fn tay() {
    // implementation
}

// TSX - Transfer Stack Pointer to X
pub fn tsx() {
    // implementation
}

// TXA - Transfer X to Accumulator
pub fn txa() {
    // implementation
}

// TXS - Transfer X to Stack Pointer
pub fn txs() {
    // implementation
}

// TYA - Transfer Y to Accumulator
pub fn tya() {
    // implementation
}

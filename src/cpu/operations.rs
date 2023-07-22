use anyhow::{bail, Result};
use log::error;
use std::fmt;

const OPCODE_SIZE_1: u16 = 1;
const OPCODE_SIZE_2: u16 = 2;
const OPCODE_SIZE_3: u16 = 3;

#[derive(Debug)]
pub enum AddressingMode {
    Absolute,
    Accumulator,
    Immediate,
    Implied,
    Relative,
    ZeroPage,
}

impl fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub enum Opcode {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Ida,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct InstructionInfo {
    pub opcode: Opcode,
    pub size: u16, // 1, 2 or 3 bytes
    pub addressing_mode: AddressingMode,
}

pub fn decode(opcode: u8) -> Result<InstructionInfo> {
    let instruction_info = match opcode {
        0x00 => InstructionInfo {
            opcode: Opcode::Brk,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0x69 => InstructionInfo {
            opcode: Opcode::Adc,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Immediate,
        },
        0x65 => InstructionInfo {
            opcode: Opcode::Adc,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0x29 => InstructionInfo {
            opcode: Opcode::And,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Immediate,
        },
        0x25 => InstructionInfo {
            opcode: Opcode::And,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0x0A => InstructionInfo {
            opcode: Opcode::Asl,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Accumulator,
        },
        0x06 => InstructionInfo {
            opcode: Opcode::Asl,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0x90 => InstructionInfo {
            opcode: Opcode::Bcc,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Relative,
        },
        0xB0 => InstructionInfo {
            opcode: Opcode::Bcs,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Relative,
        },
        0xF0 => InstructionInfo {
            opcode: Opcode::Beq,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Relative,
        },
        0x24 => InstructionInfo {
            opcode: Opcode::Bit,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0x2C => InstructionInfo {
            opcode: Opcode::Bit,
            size: OPCODE_SIZE_3,
            addressing_mode: AddressingMode::Absolute,
        },
        0x30 => InstructionInfo {
            opcode: Opcode::Bmi,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Relative,
        },
        0xD0 => InstructionInfo {
            opcode: Opcode::Bne,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Relative,
        },
        0x10 => InstructionInfo {
            opcode: Opcode::Bpl,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Relative,
        },
        0x50 => InstructionInfo {
            opcode: Opcode::Bvc,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Relative,
        },
        0x70 => InstructionInfo {
            opcode: Opcode::Bvs,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Relative,
        },
        0x18 => InstructionInfo {
            opcode: Opcode::Clc,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0xD8 => InstructionInfo {
            opcode: Opcode::Cld,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0x58 => InstructionInfo {
            opcode: Opcode::Cli,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0xB8 => InstructionInfo {
            opcode: Opcode::Clv,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0xC9 => InstructionInfo {
            opcode: Opcode::Cmp,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Immediate,
        },
        0xC5 => InstructionInfo {
            opcode: Opcode::Cmp,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0xE0 => InstructionInfo {
            opcode: Opcode::Cpx,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Immediate,
        },
        0xE4 => InstructionInfo {
            opcode: Opcode::Cpx,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0xC0 => InstructionInfo {
            opcode: Opcode::Cpy,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Immediate,
        },
        0xC4 => InstructionInfo {
            opcode: Opcode::Cpy,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0xC6 => InstructionInfo {
            opcode: Opcode::Dec,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0x49 => InstructionInfo {
            opcode: Opcode::Eor,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Immediate,
        },
        0x45 => InstructionInfo {
            opcode: Opcode::Eor,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0xE6 => InstructionInfo {
            opcode: Opcode::Inc,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0x4C => InstructionInfo {
            opcode: Opcode::Jmp,
            size: OPCODE_SIZE_3,
            addressing_mode: AddressingMode::Absolute,
        },
        0x20 => InstructionInfo {
            opcode: Opcode::Jsr,
            size: OPCODE_SIZE_3,
            addressing_mode: AddressingMode::Absolute,
        },
        0xA9 => InstructionInfo {
            opcode: Opcode::Lda,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Immediate,
        },
        0xA5 => InstructionInfo {
            opcode: Opcode::Lda,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0xA2 => InstructionInfo {
            opcode: Opcode::Ldx,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Immediate,
        },
        0xA6 => InstructionInfo {
            opcode: Opcode::Ldx,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0xA0 => InstructionInfo {
            opcode: Opcode::Ldy,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Immediate,
        },
        0xA4 => InstructionInfo {
            opcode: Opcode::Ldy,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0x4A => InstructionInfo {
            opcode: Opcode::Lsr,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Accumulator,
        },
        0x46 => InstructionInfo {
            opcode: Opcode::Lsr,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0xEA => InstructionInfo {
            opcode: Opcode::Nop,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0x09 => InstructionInfo {
            opcode: Opcode::Ora,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Immediate,
        },
        0x05 => InstructionInfo {
            opcode: Opcode::Ora,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0x48 => InstructionInfo {
            opcode: Opcode::Pha,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0x08 => InstructionInfo {
            opcode: Opcode::Php,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0x68 => InstructionInfo {
            opcode: Opcode::Pla,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0x28 => InstructionInfo {
            opcode: Opcode::Plp,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0x2A => InstructionInfo {
            opcode: Opcode::Rol,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Accumulator,
        },
        0x26 => InstructionInfo {
            opcode: Opcode::Rol,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0x6A => InstructionInfo {
            opcode: Opcode::Ror,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Accumulator,
        },
        0x66 => InstructionInfo {
            opcode: Opcode::Ror,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0x40 => InstructionInfo {
            opcode: Opcode::Rti,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0x60 => InstructionInfo {
            opcode: Opcode::Rts,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0xE9 => InstructionInfo {
            opcode: Opcode::Sbc,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::Immediate,
        },
        0xE5 => InstructionInfo {
            opcode: Opcode::Sbc,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0x38 => InstructionInfo {
            opcode: Opcode::Sec,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0xF8 => InstructionInfo {
            opcode: Opcode::Sed,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0x78 => InstructionInfo {
            opcode: Opcode::Sei,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0x85 => InstructionInfo {
            opcode: Opcode::Sta,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0x86 => InstructionInfo {
            opcode: Opcode::Stx,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0x84 => InstructionInfo {
            opcode: Opcode::Sty,
            size: OPCODE_SIZE_2,
            addressing_mode: AddressingMode::ZeroPage,
        },
        0xAA => InstructionInfo {
            opcode: Opcode::Tax,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0xA8 => InstructionInfo {
            opcode: Opcode::Tay,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0xBA => InstructionInfo {
            opcode: Opcode::Tsx,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0x8A => InstructionInfo {
            opcode: Opcode::Txa,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0x9A => InstructionInfo {
            opcode: Opcode::Txs,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0x98 => InstructionInfo {
            opcode: Opcode::Tya,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        0xE8 => InstructionInfo {
            opcode: Opcode::Inx,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Implied,
        },
        _ => {
            error!("Invalid opcode: {:#04x}", opcode);
            bail!("Invalid opcode: {:#04x}", opcode)
        }
    };
    Ok(instruction_info)
}

impl fmt::Display for InstructionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} - size - {}, mode - {}",
            self.opcode.to_string().to_ascii_uppercase(),
            self.size,
            self.addressing_mode
        )
    }
}

impl InstructionInfo {
    pub fn execute(&self) -> Result<()> {
        match self.opcode {
            Opcode::Adc => (),
            Opcode::And => (),
            Opcode::Asl => (),
            Opcode::Bcc => (),
            Opcode::Bcs => (),
            Opcode::Beq => (),
            Opcode::Bit => (),
            Opcode::Bmi => (),
            Opcode::Bne => (),
            Opcode::Bpl => (),
            Opcode::Brk => (),
            Opcode::Bvc => (),
            Opcode::Bvs => (),
            Opcode::Clc => (),
            Opcode::Cld => (),
            Opcode::Cli => (),
            Opcode::Clv => (),
            Opcode::Cmp => (),
            Opcode::Cpx => (),
            Opcode::Cpy => (),
            Opcode::Dec => (),
            Opcode::Dex => (),
            Opcode::Dey => (),
            Opcode::Eor => (),
            Opcode::Inc => (),
            Opcode::Inx => (),
            Opcode::Iny => (),
            Opcode::Jmp => (),
            Opcode::Jsr => (),
            Opcode::Ida => (),
            Opcode::Lda => (),
            Opcode::Ldx => (),
            Opcode::Ldy => (),
            Opcode::Lsr => (),
            Opcode::Nop => (),
            Opcode::Ora => (),
            Opcode::Pha => (),
            Opcode::Php => (),
            Opcode::Pla => (),
            Opcode::Plp => (),
            Opcode::Rol => (),
            Opcode::Ror => (),
            Opcode::Rti => (),
            Opcode::Rts => (),
            Opcode::Sbc => (),
            Opcode::Sec => (),
            Opcode::Sed => (),
            Opcode::Sei => (),
            Opcode::Sta => (),
            Opcode::Stx => (),
            Opcode::Sty => (),
            Opcode::Tax => (),
            Opcode::Tay => (),
            Opcode::Tsx => (),
            Opcode::Txa => (),
            Opcode::Txs => (),
            Opcode::Tya => (),
        }
        Ok(())
    }
}

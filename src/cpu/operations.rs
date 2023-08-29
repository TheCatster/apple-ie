use anyhow::{bail, Result};
use log::{error, info};
use std::{
    fmt,
    hash::{Hash, Hasher},
};

const OPCODE_SIZE_1: u16 = 1;
const OPCODE_SIZE_2: u16 = 2;
const OPCODE_SIZE_3: u16 = 3;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
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

impl PartialEq for AddressingMode {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl Hash for AddressingMode {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        format!("{:?}", self).hash(hasher)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
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
        write!(f, "{}", format!("{:?}", self).to_uppercase())
    }
}

impl PartialEq for Opcode {
    fn eq(&self, other: &Self) -> bool {
        format!("{}", self) == format!("{}", other)
    }
}

impl Hash for Opcode {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        format!("{:?}", self).hash(hasher)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct InstructionInfo {
    pub opcode: Opcode,
    pub opcode_value: u8,
    pub size: u16, // 1, 2 or 3 bytes
    pub addressing_mode: AddressingMode,
}

impl fmt::Display for InstructionInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} - size - {}, mode - {}",
            self.opcode, self.size, self.addressing_mode
        )
    }
}

impl PartialEq for InstructionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.opcode == other.opcode
            && self.size == other.size
            && self.addressing_mode == other.addressing_mode
    }
}

impl Hash for InstructionInfo {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        format!("{:?}", self).hash(hasher)
    }
}

pub static INSTRUCTIONS: [InstructionInfo; 69] = [
    InstructionInfo {
        opcode: Opcode::Brk,
        opcode_value: 0x00,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Adc,
        opcode_value: 0x69,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Adc,
        opcode_value: 0x65,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::And,
        opcode_value: 0x29,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::And,
        opcode_value: 0x25,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Asl,
        opcode_value: 0x0A,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Accumulator,
    },
    InstructionInfo {
        opcode: Opcode::Asl,
        opcode_value: 0x06,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Bcc,
        opcode_value: 0x90,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Bcs,
        opcode_value: 0xB0,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Beq,
        opcode_value: 0xF0,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Bit,
        opcode_value: 0x24,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Bit,
        opcode_value: 0x2C,
        size: OPCODE_SIZE_3,
        addressing_mode: AddressingMode::Absolute,
    },
    InstructionInfo {
        opcode: Opcode::Bmi,
        opcode_value: 0x30,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Bne,
        opcode_value: 0xD0,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Bpl,
        opcode_value: 0x10,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Bvc,
        opcode_value: 0x50,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Bvs,
        opcode_value: 0x70,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Clc,
        opcode_value: 0x18,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Cld,
        opcode_value: 0xD8,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Cli,
        opcode_value: 0x58,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Clv,
        opcode_value: 0xB8,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Cmp,
        opcode_value: 0xC9,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Cmp,
        opcode_value: 0xC5,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Cpx,
        opcode_value: 0xE0,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Cpx,
        opcode_value: 0xE4,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Cpy,
        opcode_value: 0xC0,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Cpy,
        opcode_value: 0xC4,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Dec,
        opcode_value: 0xC6,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Eor,
        opcode_value: 0x49,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Eor,
        opcode_value: 0x45,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Inc,
        opcode_value: 0xE6,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Jmp,
        opcode_value: 0x4C,
        size: OPCODE_SIZE_3,
        addressing_mode: AddressingMode::Absolute,
    },
    InstructionInfo {
        opcode: Opcode::Jsr,
        opcode_value: 0x20,
        size: OPCODE_SIZE_3,
        addressing_mode: AddressingMode::Absolute,
    },
    InstructionInfo {
        opcode: Opcode::Lda,
        opcode_value: 0xA9,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Lda,
        opcode_value: 0xA5,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Ldx,
        opcode_value: 0xA2,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Ldx,
        opcode_value: 0xA6,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Ldy,
        opcode_value: 0xA0,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Ldy,
        opcode_value: 0xA4,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Lsr,
        opcode_value: 0x4A,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Accumulator,
    },
    InstructionInfo {
        opcode: Opcode::Lsr,
        opcode_value: 0x46,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Nop,
        opcode_value: 0xEA,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Ora,
        opcode_value: 0x09,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Ora,
        opcode_value: 0x05,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Pha,
        opcode_value: 0x48,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Php,
        opcode_value: 0x08,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Pla,
        opcode_value: 0x68,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Plp,
        opcode_value: 0x28,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Rol,
        opcode_value: 0x2A,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Accumulator,
    },
    InstructionInfo {
        opcode: Opcode::Rol,
        opcode_value: 0x26,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Ror,
        opcode_value: 0x6A,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Accumulator,
    },
    InstructionInfo {
        opcode: Opcode::Ror,
        opcode_value: 0x66,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Rti,
        opcode_value: 0x40,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Rts,
        opcode_value: 0x60,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Sbc,
        opcode_value: 0xE9,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Sbc,
        opcode_value: 0xE5,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Sec,
        opcode_value: 0x38,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Sed,
        opcode_value: 0xF8,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Sei,
        opcode_value: 0x78,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Sta,
        opcode_value: 0x85,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Stx,
        opcode_value: 0x86,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Sty,
        opcode_value: 0x84,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Tax,
        opcode_value: 0xAA,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Tay,
        opcode_value: 0xA8,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Tsx,
        opcode_value: 0xBA,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Txa,
        opcode_value: 0x8A,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Txs,
        opcode_value: 0x9A,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Tya,
        opcode_value: 0x98,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Inx,
        opcode_value: 0xE8,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
];

pub fn get_instruction(
    code: Option<u8>,
    name: Option<&str>,
    addressing: Option<AddressingMode>,
) -> Option<InstructionInfo> {
    match code {
        Some(code) => {
            info!("Searching by code.");
            let instruction_info = INSTRUCTIONS.iter().find(|o| o.opcode_value == code);
            return match instruction_info {
                Some(instruction_info) => Some(*instruction_info),
                None => {
                    error!("Unknown operation code: 0x{:X}", code);
                    None
                }
            };
        }
        None => (),
    };

    let remaining_args: (Option<&str>, Option<AddressingMode>) = (name, addressing);

    match remaining_args {
        (Some(name), Some(mode)) => {
            info!("Searching by name and mode.");
            let name = name.to_uppercase();
            Some(
                INSTRUCTIONS
                    .into_iter()
                    .find(|o| o.opcode.to_string() == name && o.addressing_mode == mode)
                    .expect("unknown operation name"),
            )
        }
        _ => None,
    }
}

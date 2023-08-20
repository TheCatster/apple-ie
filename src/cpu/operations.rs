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
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Adc,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Adc,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::And,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::And,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Asl,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Accumulator,
    },
    InstructionInfo {
        opcode: Opcode::Asl,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Bcc,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Bcs,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Beq,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Bit,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Bit,
        size: OPCODE_SIZE_3,
        addressing_mode: AddressingMode::Absolute,
    },
    InstructionInfo {
        opcode: Opcode::Bmi,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Bne,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Bpl,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Bvc,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Bvs,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Relative,
    },
    InstructionInfo {
        opcode: Opcode::Clc,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Cld,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Cli,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Clv,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Cmp,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Cmp,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Cpx,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Cpx,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Cpy,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Cpy,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Dec,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Eor,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Eor,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Inc,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Jmp,
        size: OPCODE_SIZE_3,
        addressing_mode: AddressingMode::Absolute,
    },
    InstructionInfo {
        opcode: Opcode::Jsr,
        size: OPCODE_SIZE_3,
        addressing_mode: AddressingMode::Absolute,
    },
    InstructionInfo {
        opcode: Opcode::Lda,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Lda,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Ldx,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Ldx,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Ldy,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Ldy,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Lsr,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Accumulator,
    },
    InstructionInfo {
        opcode: Opcode::Lsr,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Nop,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Ora,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Ora,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Pha,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Php,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Pla,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Plp,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Rol,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Accumulator,
    },
    InstructionInfo {
        opcode: Opcode::Rol,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Ror,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Accumulator,
    },
    InstructionInfo {
        opcode: Opcode::Ror,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Rti,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Rts,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Sbc,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::Immediate,
    },
    InstructionInfo {
        opcode: Opcode::Sbc,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Sec,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Sed,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Sei,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Sta,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Stx,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Sty,
        size: OPCODE_SIZE_2,
        addressing_mode: AddressingMode::ZeroPage,
    },
    InstructionInfo {
        opcode: Opcode::Tax,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Tay,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Tsx,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Txa,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Txs,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Tya,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
    InstructionInfo {
        opcode: Opcode::Inx,
        size: OPCODE_SIZE_1,
        addressing_mode: AddressingMode::Implied,
    },
];

pub fn get_instruction(
    code: Option<u8>,
    name: Option<&str>,
    addressing: Option<AddressingMode>,
) -> Result<InstructionInfo> {
    if code.is_some() {
        info!("Searching by code.");

        let code = if let Some(code) = code {
            code
        } else {
            bail!("No code provided! How??")
        };

        let instruction_info = INSTRUCTIONS.iter().find(|o| o.opcode as u8 == code);

        if let Some(instruction_info) = instruction_info {
            Ok(*instruction_info)
        } else {
            error!("Unknown operation code: 0x{:X}", code);
            bail!("Unknown operation code!");
        }
    } else if addressing.is_none() {
        info!("Searching by name.");

        let name = if let Some(name) = name {
            name
        } else {
            bail!("No name provided!")
        };

        let instruction_info_list: Vec<InstructionInfo> = INSTRUCTIONS
            .into_iter()
            .filter(|&o| o.opcode.to_string() == name)
            .collect::<Vec<InstructionInfo>>();

        if let Some(instruction_info) = instruction_info_list.first() {
            Ok(*instruction_info)
        } else {
            bail!("No values in the instruction info list!")
        }
    } else if name.is_some() {
        info!("Searching by name and mode.");

        let name = if let Some(name) = name {
            name.to_uppercase()
        } else {
            bail!("No name provided!")
        };

        let mode = if let Some(addressing) = addressing {
            addressing
        } else {
            bail!("No name provided!")
        };

        Ok(INSTRUCTIONS
            .into_iter()
            .find(|o| o.opcode.to_string() == name && o.addressing_mode == mode)
            .expect("unknown operation name"))
    } else {
        error!("Operation does not exist!");
        bail!("Operation does not exist!");
    }
}

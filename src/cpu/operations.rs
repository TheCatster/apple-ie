pub const OPCODE_SIZE_1: u16 = 1;
pub const OPCODE_SIZE_2: u16 = 2;
pub const _OPCODE_SIZE_3: u16 = 3;

pub enum AddressingMode {
    Immediate,
    Implied,
    _Absolute,
    ZeroPage,
}

pub struct InstructionInfo {
    pub opcode: Opcode,
    pub size: u16, // 1, 2 or 3 bytes
    pub addressing_mode: AddressingMode,
}

pub fn decode(opcode: u8) -> InstructionInfo {
    match opcode {
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
        0xEA => InstructionInfo {
            opcode: Opcode::Nop,
            size: OPCODE_SIZE_1,
            addressing_mode: AddressingMode::Immediate,
        },
        // ... add other opcodes
        _ => panic!("Invalid opcode: {}", opcode),
    }
}

pub enum Opcode {
    Adc,
    Brk,
    _And,
    _Asl,
    _Sta,
    Nop,
}

impl Opcode {
    pub fn execute(&self) {
        match self {
            Self::Adc => todo!(),
            Self::Brk => todo!(),
            _ => (),
        }
    }
}

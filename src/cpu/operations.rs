pub const OPCODE_SIZE_1: u16 = 1;
pub const OPCODE_SIZE_2: u16 = 2;
pub const _OPCODE_SIZE_3: u16 = 3;

pub enum AddressingMode {
    Immediate,
    Implied,
    _Absolute,
    ZeroPage,
}

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

impl InstructionInfo {
    pub fn execute(&mut self) {
        match self.opcode {
            Opcode::Adc => todo!(),
            Opcode::And => todo!(),
            Opcode::Asl => todo!(),
            Opcode::Bcc => todo!(),
            Opcode::Bcs => todo!(),
            Opcode::Beq => todo!(),
            Opcode::Bit => todo!(),
            Opcode::Bmi => todo!(),
            Opcode::Bne => todo!(),
            Opcode::Bpl => todo!(),
            Opcode::Brk => todo!(),
            Opcode::Bvc => todo!(),
            Opcode::Bvs => todo!(),
            Opcode::Clc => todo!(),
            Opcode::Cld => todo!(),
            Opcode::Cli => todo!(),
            Opcode::Clv => todo!(),
            Opcode::Cmp => todo!(),
            Opcode::Cpx => todo!(),
            Opcode::Cpy => todo!(),
            Opcode::Dec => todo!(),
            Opcode::Dex => todo!(),
            Opcode::Dey => todo!(),
            Opcode::Eor => todo!(),
            Opcode::Inc => todo!(),
            Opcode::Inx => todo!(),
            Opcode::Iny => todo!(),
            Opcode::Jmp => todo!(),
            Opcode::Jsr => todo!(),
            Opcode::Ida => todo!(),
            Opcode::Ldx => todo!(),
            Opcode::Ldy => todo!(),
            Opcode::Lsr => todo!(),
            Opcode::Nop => todo!(),
            Opcode::Ora => todo!(),
            Opcode::Pha => todo!(),
            Opcode::Php => todo!(),
            Opcode::Pla => todo!(),
            Opcode::Plp => todo!(),
            Opcode::Rol => todo!(),
            Opcode::Ror => todo!(),
            Opcode::Rti => todo!(),
            Opcode::Rts => todo!(),
            Opcode::Sbc => todo!(),
            Opcode::Sec => todo!(),
            Opcode::Sed => todo!(),
            Opcode::Sei => todo!(),
            Opcode::Sta => todo!(),
            Opcode::Stx => todo!(),
            Opcode::Sty => todo!(),
            Opcode::Tax => todo!(),
            Opcode::Tay => todo!(),
            Opcode::Tsx => todo!(),
            Opcode::Txa => todo!(),
            Opcode::Txs => todo!(),
            Opcode::Tya => todo!(),
        }
    }
}

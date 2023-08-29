use crate::memory::Memory;
use operations::{get_instruction, AddressingMode, InstructionInfo, Opcode};

use anyhow::{bail, Result};
use bitflags::bitflags;
use log::info;
use std::{thread, time};

pub mod operations;

const _CPU_CLOCK_RATE: u32 = 1_000_000; // 1 MHz
const _DEFAULT_FLAGS: u8 = 0b0011_0000;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct StatusFlags: u8 {
        const CARRY      = 0b0000_0001;
        const ZERO       = 0b0000_0010;
        const INTERRUPT  = 0b0000_0100;
        const DECIMAL    = 0b0000_1000;
        const BREAK      = 0b0001_0000;
        const UNUSED     = 0b0010_0000;
        const OVERFLOW   = 0b0100_0000;
        const NEGATIVE   = 0b1000_0000;
        const DEFAULT    = Self::UNUSED.bits() | Self::BREAK.bits();
    }
}

pub struct Registers {
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub program_counter: u16,
    pub status: StatusFlags,
    pub stack_pointer: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            accumulator: 0,
            x: 0,
            y: 0,
            program_counter: 0,
            status: StatusFlags::DEFAULT,
            stack_pointer: 0,
        }
    }
}

pub struct Cpu {
    registers: Registers,
    memory: Memory,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        info!("Beginning main F-D-E loop.");
        loop {
            self.fde()?
        }
    }

    pub fn load(&mut self, address: u16, buffer: &[u8]) {
        self.memory.load(address, buffer);
        self.registers.program_counter = address;
    }

    pub fn execute(&mut self, instruction_info: &InstructionInfo) -> Result<()> {
        match instruction_info.opcode {
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
            Opcode::Lda => match instruction_info.addressing_mode {
                AddressingMode::Immediate => {
                    self.registers.accumulator = self.fetch_byte()?;
                    self.registers.status.toggle(StatusFlags::ZERO);
                    self.registers.status.set(
                        StatusFlags::NEGATIVE,
                        (self.registers.status & StatusFlags::NEGATIVE).bits() > 0,
                    );
                }
                _ => (),
            },
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

        info!(
            "Instruction {} executed!",
            instruction_info.opcode.to_string().to_uppercase()
        );

        Ok(())
    }

    pub fn fde(&mut self) -> Result<()> {
        // Fetch
        let instruction = self.fetch_byte()?;

        // Decode
        let instruction_info = self.decode(instruction)?;

        self.registers.program_counter += instruction_info.size;
        info!(
            "Program counter incremented: {:#04x}",
            self.registers.program_counter
        );

        // Execute
        // For testing, if a BRK, then exit
        if instruction_info.opcode == Opcode::Brk {
            bail!("BRK hit!");
        };

        self.execute(&instruction_info)?;

        // Slow down for now
        thread::sleep(time::Duration::from_millis(1000));

        Ok(())
    }

    fn fetch_byte(&mut self) -> Result<u8> {
        info!(
            "Current address is: {:#04x}",
            self.registers.program_counter
        );

        let byte = match self.memory.read(self.registers.program_counter) {
            Some(byte) => byte,
            None => bail!("Cannot read next value in memory!"),
        };

        info!("Byte retrieved: {:#04x}", &byte);

        Ok(byte)
    }

    fn decode(&mut self, instruction: u8) -> Result<InstructionInfo> {
        let instruction_info = match get_instruction(Some(instruction), None, None) {
            Some(x) => x,
            None => bail!("Instruction was invalid!"),
        };

        info!("Instruction decoded: {}", &instruction_info);

        Ok(instruction_info)
    }

    pub fn _get_status(&self, status: StatusFlags) -> bool {
        self.registers.status.bits() & status.bits() != 0
    }

    pub fn set_status(&mut self, status: StatusFlags, value: bool) {
        if value {
            self.registers.status |= status;
        } else {
            self.registers.status &= !status;
        }
    }
}

use crate::memory::Memory;
use instruction_info::{AddressingMode, InstructionInfo, Opcode};

use bitflags::bitflags;
use log::info;

mod instruction_info;

pub const CPU_CLOCK_RATE: u32 = 1_000_000; // 1 MHz
pub const OPCODE_SIZE_1: usize = 1;
pub const OPCODE_SIZE_2: usize = 2;
pub const OPCODE_SIZE_3: usize = 3;
pub const DEFAULT_FLAGS: u8 = 0b0011_0000;

bitflags! {
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

pub struct CPU {
    registers: Registers,
    memory: Memory,
    clock: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
            memory: Memory::new(),
            clock: 0,
        }
    }

    pub fn run(&mut self) {
        info!("Beginning main F-D-E loop.");
        loop {
            self.fde()
        }
    }

    pub fn load(&mut self, address: u16, buffer: &[u8]) {
        self.memory.load(address, buffer);
        self.registers.program_counter = address;
    }

    pub fn execute(&mut self, instruction_info: &InstructionInfo) {}

    pub fn fde(&mut self) {
        // Fetch
        info!(
            "Current address is: {:#04x}",
            self.registers.program_counter
        );
        let instruction = self
            .memory
            .read(self.registers.program_counter)
            .unwrap_or(0xEA);

        if instruction != 0xEA {
            info!("Instruction retrieved: {:#04x}", &instruction);

            // Decode
            let instruction_info = self.decode(instruction);

            info!("Instruction decoded!");

            // Execute
            self.execute(&instruction_info);

            info!("Instruction executed!");

            // Increment program counter
            self.registers.program_counter += instruction as u16;
            info!("Program counter incremented!");
        }
    }

    pub fn get_status(&self, status: StatusFlags) -> bool {
        self.registers.status.bits() & status.bits() != 0
    }

    pub fn set_status(&mut self, status: StatusFlags, value: bool) {
        if value {
            self.registers.status |= status;
        } else {
            self.registers.status &= !status;
        }
    }

    pub fn decode(&mut self, opcode: u8) -> InstructionInfo {
        match opcode {
            0x00 => InstructionInfo {
                opcode: Opcode::BRK,
                size: 0,
                addressing_mode: AddressingMode::Implied,
                cycle_count: 1,
            },
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
            0xEA => InstructionInfo {
                opcode: Opcode::NOP,
                size: 0,
                addressing_mode: AddressingMode::Immediate,
                cycle_count: 1,
            },
            // ... add other opcodes
            _ => panic!("Invalid opcode: {}", opcode),
        }
    }
}

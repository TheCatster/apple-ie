use crate::memory::Memory;
use operations::{decode, InstructionInfo};

use anyhow::Result;
use bitflags::bitflags;
use log::info;
use std::{thread, time};

mod operations;

const _CPU_CLOCK_RATE: u32 = 1_000_000; // 1 MHz
const _DEFAULT_FLAGS: u8 = 0b0011_0000;

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
        instruction_info.execute()?;
        Ok(())
    }

    pub fn fde(&mut self) -> Result<()> {
        // Fetch
        info!(
            "Current address is: {:#04x}",
            self.registers.program_counter
        );
        let instruction = self
            .memory
            .read(self.registers.program_counter)
            .unwrap_or(0xEA);

        info!("Instruction retrieved: {:#04x}", &instruction);

        // Decode
        let instruction_info = decode(instruction)?;

        info!("Instruction decoded: {}", &instruction_info);

        // Execute
        self.execute(&instruction_info)?;

        info!("Instruction executed!");

        // Increment program counter
        self.registers.program_counter += instruction_info.size;
        info!("Program counter incremented!");

        // Slow down for now
        thread::sleep(time::Duration::from_millis(1000));

        Ok(())
    }

    pub fn _get_status(&self, status: StatusFlags) -> bool {
        self.registers.status.bits() & status.bits() != 0
    }

    pub fn _set_status(&mut self, status: StatusFlags, value: bool) {
        if value {
            self.registers.status |= status;
        } else {
            self.registers.status &= !status;
        }
    }
}

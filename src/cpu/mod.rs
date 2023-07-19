use crate::assembler::{adc, and, asl, InstructionInfo, Opcode};
use crate::memory::Memory;

use bitflags::bitflags;

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

struct Registers {
    accumulator: u8,
    x: u8,
    y: u8,
    program_counter: u16,
    status: StatusFlags,
    stack_pointer: u8,
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
        loop {
            self.execute()
        }
    }

    pub fn load(&mut self, address: u16, buffer: &[u8]) {
        self.memory.load(address, buffer);
        self.registers.program_counter = address;
    }

    pub fn execute(&mut self) {
        // Fetch
        let opcode = self.memory.read(self.registers.program_counter);

        // Decode
        let instruction = decode(opcode);

        // Execute
        //self.execute(&instruction.opcode);

        // Increment program counter
        self.registers.program_counter += instruction.size as u16;
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
}

use crate::memory::Memory;
use crate::assembler::{adc, and, asl, InstructionInfo, Opcode};
use bitflags::bitflags;

bitflags! {
    pub struct StatusFlags {
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
    stack_pointer: u8,
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            accumulator: 0,
            x: 0,
            y: 0,
            program_counter: 0,
            stack_pointer: 0,
        }
    }
}

pub struct CPU {
    registers: Registers,
    status: StatusFlags,
    memory: Memory,
    clock: u8, 
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
            status: StatusFlags::default(),
            memory: Memory::new(),
            clock: 0,
        }
    }
    pub fn begin(&mut self) {
        loop {
            // fetch, decode, execute
        }
    }

    fn execute(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::ADC => adc(),
            Opcode::AND => and(),
            Opcode::ASL => asl(),
            // ...
        }
    }

    pub fn get_status(&self, status: StatusFlags) -> bool {
        self.status.bits() & status.bits() != 0
    }

    pub fn set_status(&mut self, status: StatusFlags, value: bool) {
        if value {
            self.status |= status;
        } else {
            self.status &= !status;
        }
    }
}
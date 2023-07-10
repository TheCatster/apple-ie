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

pub struct CPUState {
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub status: StatusFlags,
    pub memory: Memory,
    pub clock: u8,
}

impl CPUState {
    pub fn new() -> CPUState {
        CPUState {
            accumulator: 0,
            x: 0,
            y: 0,
            program_counter: 0,
            stack_pointer: 0,
            status: StatusFlags::DEFAULT,
            memory: Memory::new(),
            clock: 0,
        }
    }

    pub fn begin(&mut self, instruction_set: &[u8]) {
        loop {
            // Fetch
            let opcode = memory.read(self.program_counter);
            self.program_counter += instruction_set[opcode as usize].size;

            // Decode
            let instruction = &instruction_set[opcode as usize];

            // Execute
            match instruction.opcode {
                Opcode::ADC => self.adc(&memory, instruction.addressing_mode),
                Opcode::STA => self.sta(&memory, instruction.addressing_mode),
                Opcode::JMP => self.jmp(&memory, instruction.addressing_mode),
                // etc...
            }

            // Repeat for cycle count
            for _ in 0..instruction.cycle_count {
                self.clock += 1;
            }
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

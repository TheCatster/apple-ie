pub struct Memory {
    pub ram: [u8; 48 * 1024], // 8KB ROM
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: [0; 48 * 1024],
        }
    }

    pub fn read(&self, address: u16) -> Option<u8> {
        self.ram.get(address as usize).copied()
    }

    pub fn load(&mut self, address: u16, buffer: &[u8]) {
        for (i, byte) in buffer.iter().enumerate() {
            self.ram[address as usize + i] = *byte;
        }
    }
}

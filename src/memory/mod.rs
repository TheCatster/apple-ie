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

    pub fn load(&mut self, _address: u16, _buffer: &[u8]) {}
}

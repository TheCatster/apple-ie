pub struct Memory {
    pub ram: [u8; 0x800],  // 2KB RAM
    pub rom: [u8; 0x2000], // 8KB ROM
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: [0; 0x800],
            rom: [0; 0x2000],
        }
    }
}
    pub fn read(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

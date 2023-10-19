pub struct Memory {
    pub ram: [u8; 48 * 1024], // 48KB ROM
}

impl Memory {
    /// Create a new Memory instance with 48KB of RAM.
    ///
    /// # Returns
    ///
    /// A new `Memory` instance with 48KB of RAM.
    ///
    /// # Examples
    ///
    /// ```
    /// let memory = Memory::new();
    /// assert_eq!(memory.ram.len(), 48 * 1024);
    /// ```
    pub fn new() -> Memory {
        Memory {
            ram: [0; 48 * 1024],
        }
    }

    /// Read a byte from the specified address in memory.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to read the byte from.
    ///
    /// # Returns
    ///
    /// An `Option` containing the byte at the specified address, or `None` if the address is out of range.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut memory = Memory::new();
    /// memory.ram[0x1234] = 0xAB;
    /// assert_eq!(memory.read(0x1234), Some(0xAB));
    /// ```
    pub fn read(&self, address: u16) -> Option<u8> {
        // Use the `get` method to get the byte at the specified address
        // and return it as an `Option`
        self.ram.get(address as usize).copied()
    }

    /// Load a buffer of bytes into memory starting at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to start loading the buffer into.
    /// * `buffer` - The buffer of bytes to load into memory.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut memory = Memory::new();
    /// let buffer = vec![0xAB, 0xCD, 0xEF];
    /// memory.load(0x1234, &buffer);
    /// assert_eq!(memory.ram[0x1234], 0xAB);
    /// assert_eq!(memory.ram[0x1235], 0xCD);
    /// assert_eq!(memory.ram[0x1236], 0xEF);
    /// ```
    pub fn load(&mut self, address: u16, buffer: &[u8]) {
        // Iterate over the buffer and copy each byte into memory
        for (i, byte) in buffer.iter().enumerate() {
            self.ram[address as usize + i] = *byte;
        }
    }
}
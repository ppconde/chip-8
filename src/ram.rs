const MEM_SIZE: usize = 4096;
pub struct Ram {
    memory: [u8; MEM_SIZE],
}

// Needs manual default implementation
impl Default for Ram {
    fn default() -> Self {
        Self {
            memory: [0; MEM_SIZE],
        } // Explicitly initialize the array
    }
}

impl Ram {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

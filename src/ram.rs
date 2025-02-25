pub struct Ram {
    memory: [u8; 4096],
}

// Needs manual default implementation
impl Default for Ram {
    fn default() -> Self {
        Self { memory: [0; 4096] } // Explicitly initialize the array
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

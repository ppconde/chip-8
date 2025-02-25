use crate::ram::Ram;

#[derive(Default)] // Enables `Default::default()`
struct Chip8 {
    memory: Ram, // 4KB RAM

    // Registers
    v: [u8; 16], // 16 general purpose registers
    i: u16, // This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
    // When these registers are non-zero, they are automatically decremented at a rate of 60Hz
    delay_timer: u8,
    sound_timer: u8,
    pc: u16, // Program counter - used to store the currently executing address
    sp: u8,  // Stack pointer - used to point to the topmost level of the stack
    stack: [u16; 16],

    // screen: [bool; 64 * 32], // Monochrome display
    // keys: [bool; 16],        // Hex keypad state
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            pc: 0x200, // Set program counter to 0x200
            memory: [0; 4096],
            // ..Default::default() // Use default values for everything else
        };

        // chip8.load_fontset();
        // chip8
    }


    fn load_rom(&mut self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        for (i, &byte) in buffer.iter().enumerate() {
            self.memory[0x200 + i] = byte;
        }
        Ok(())
    }

     fn load_fontset(&mut self) {
        let font_set: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];

        self.memory[]
    }
}

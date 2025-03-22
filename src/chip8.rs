use crate::ram::Ram;
use rand::Rng;

const NUM_REGS: usize = 16;
const STACK_SIZE: usize = 16;
const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const SCREEN_SIZE: usize = SCREEN_HEIGHT * SCREEN_WIDTH;
const NUM_KEYS: usize = 16;
const START_ADDR: u16 = 0x200;

pub struct Chip8 {
    memory: Ram, // 4KB RAM
    // Registers
    v: [u8; NUM_REGS], // 16 general purpose registers
    i: u16, // This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
    // When these registers are non-zero, they are automatically decremented at a rate of 60Hz
    delay_timer: u8,
    sound_timer: u8,
    pc: u16, // Program counter - used to store the currently executing address
    sp: u8,  // Stack pointer - used to point to the topmost level of the stack
    stack: [u16; STACK_SIZE],

    screen: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT], // Monochrome display
    keys: [bool; NUM_KEYS],                      // Hex keypad state
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Chip8 {
            memory: Ram::default(),
            v: [0; NUM_REGS],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            sp: 0,
            stack: [0; STACK_SIZE],
            screen: [[0u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
            keys: [false; NUM_KEYS],
        };

        chip8.load_fontset();

        chip8
    }

    pub fn load_rom(&mut self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(filename)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        for (i, &byte) in buffer.iter().enumerate() {
            self.memory
                .write_byte(((START_ADDR as usize) + i) as u16, byte);
        }
        Ok(())
    }

    pub fn run_cycle(&mut self) {
        // 1. Fetch the next opcode (CHIP-8 opcodes are 2 bytes)
        let op_code = self.fetch_opcode();
        let x = ((op_code >> 8) & 0xF) as usize; //lower 4 bits of first byte
        let y = ((op_code >> 4) & 0xF) as usize; //higher 4 bits of second byte
        let n = (op_code & 0xF) as u8; //lower 4 bits of second byte
        let nnn = (op_code & 0xFFF) as u16; //lower 12 bits of opcode
        let kk = (op_code & 0xFF) as u8; //lower bits 8 of opcode

        match op_code {
            // CLS
            0x00E0 => self._00E0(),
            // RET
            0x00EE => self._00EE(),
            // JP addr
            0x1000 => self._1nnn(nnn),
            0x2000 => self._2nnn(nnn),
            0x3000 => self._3xkk(x, kk),
            0x4000 => self._4xkk(x, kk),
            0x5000 => self._5xy0(x, y),
            0x6000 => self._6xkk(x, kk),
            0x7000 => self._7kkk(x, kk),
            _ => {
                eprintln!("Unknown opcode: {:#X}", op_code)
            }
        }
    }

    fn fetch_opcode(&self) -> u16 {
        let byte1 = self.memory.read_byte(self.pc) as u16;
        let byte2 = self.memory.read_byte(self.pc + 1) as u16;

        // Basically shifts to the left and adds byte2 to the rightmost side of byte1
        // Opcodes are 16 bits long
        (byte1) << 8 | byte2
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

        // Store font into memory RAM
        for (i, &byte) in font_set.iter().enumerate() {
            self.memory.write_byte((0x050 + i) as u16, byte);
        }
    }

    fn _00E0(&mut self) {
        self.screen = [false; SCREEN_SIZE]
    }

    fn _00EE(&mut self) {
        if self.sp == 0 {
            panic!("Stack underflow: Attempted to return with an empty stack");
        }
        self.pc = self.stack[self.pc as usize];
        self.pc -= 1;
    }

    fn _1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn _2nnn(&mut self, nnn: u16) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = nnn;
    }

    fn _3xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] == kk {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn _4xkk(&mut self, x: usize, kk: u8) {
        if self.v[x] != kk {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn _5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    fn _6xkk(&mut self, x: usize, kk: u8) {
        self.v[x] = kk;
        self.pc += 2;
    }

    fn _7kkk(&mut self, x: usize, kk: u8) {
        self.v[x] = self.v[x].wrapping_add(kk);
        self.pc += 2;
    }

    fn _8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
        self.pc += 2;
    }

    fn _8xy1(&mut self, x: usize, y: usize) {
        // Bitwise OR
        self.v[x] |= self.v[y];
        self.pc += 2;
    }

    fn _8xy2(&mut self, x: usize, y: usize) {
        // Bitwise AND
        self.v[x] &= self.v[y];
        self.pc += 2;
    }

    fn _8xy3(&mut self, x: usize, y: usize) {
        // Bitwise XOR
        self.v[x] ^= self.v[y];
        self.pc += 2;
    }

    fn _8xy4(&mut self, x: usize, y: usize) {
        // 8xy4 - ADD Vx, Vy
        // Set Vx = Vx + Vy, set VF = carry.

        // The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,)
        // VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
        let (result, carry) = (self.v[x]).overflowing_add(self.v[y]);
        self.v[x] = result;
        self.v[0xF] = if carry { 1 } else { 0 };
        self.pc += 2;
    }

    fn _8xy5(&mut self, x: usize, y: usize) {
        // Set Vx = Vx - Vy, set VF = NOT borrow.

        // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
        self.v[0xF] = if self.v[x] > self.v[y] { 1 } else { 0 };
        self.v[x] = self.v[x].wrapping_sub(self.v[y]);
        self.pc += 2;
    }

    fn _8xy6(&mut self, x: usize, y: usize) {
        // Set Vx = Vx SHR 1.

        // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
        // First check and save the LSB of Vx to VF
        self.v[0xF] = self.v[x] & 0x1;
        // Then shift Vx right (which is the same as dividing by 2)
        self.v[x] >>= 1;
        self.pc += 2;
    }

    fn _8xy7(&mut self, x: usize, y: usize) {
        // Set Vx = Vy - Vx, set VF = NOT borrow.

        // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
        self.v[0xF] = if self.v[y] > self.v[x] { 1 } else { 0 };
        self.v[x] = self.v[y].wrapping_sub(self.v[x]);
        self.pc += 2;
    }

    fn _8xy_e(&mut self, x: usize) {
        // Set Vx = Vx SHL 1.

        // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
        self.v[0xF] = if self.v[x] & 0x80 == 0x80 { 1 } else { 0 };
        self.v[x] <<= 1;
        self.pc += 2;
    }

    fn _9xy0(&mut self, x: usize, y: usize) {
        // Skip next instruction if Vx != Vy.

        // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
        if self.v[x] != self.v[y] {
            self.pc += 2
        }
        self.pc += 2;
    }

    fn _annn(&mut self, nnn: u16) {
        // Set I = nnn.

        // The value of register I is set to nnn.
        self.i = nnn;
        self.pc += 2;
    }

    fn _bnnn(&mut self, nnn: u16) {
        // Jump to location nnn + V0.

        // The program counter is set to nnn plus the value of V0.
        self.pc = nnn + self.v[0] as u16;
    }

    fn _cxkk(&mut self, x: usize, kk: u8) {
        // Set Vx = random byte AND kk.

        // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk.
        // The results are stored in Vx. See instruction 8xy2 for more information on AND.

        self.v[x] = rand::rng().random_range(0..=255) & kk;
        self.pc += 2;
    }

    fn _dxyn(&mut self, x: usize, y: usize, n: u8) {
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
        let x_coord = self.v[x] as usize;
        let y_coord = self.v[y] as usize;
        self.v[0xF] = 0; // Reset collision flag

        for row in 0..n as usize {
            // Get one row of sprite data (8 pixels wide)
            let sprite_data = self.memory.read_byte(self.i + row as u16);

            // The y-coordinate wraps around the screen
            let y_pos = (y_coord + row) % SCREEN_HEIGHT;

            // Process each bit in the sprite row (8 bits = 8 pixels wide)
            for col in 0..8 {
                // Only draw if the sprite bit is 1 (MSB first, so 7-col)
                let sprite_bit = (sprite_data >> (7 - col)) & 0x1;

                if sprite_bit != 0 {
                    // The x-coordinate wraps around the screen
                    let x_pos = (x_coord + col) % SCREEN_WIDTH;

                    // Toggle the pixel and check for collision
                    if self.screen[y_pos][x_pos] == 1 {
                        // Collision detected - a pixel was turned off
                        self.v[0xF] = 1;
                    }

                    // XOR the existing pixel with the sprite bit
                    self.screen[y_pos][x_pos] ^= 1;
                }
            }
        }

        self.pc += 2;
    }
}

use crate::ram::Ram;

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

    screen: [bool; SCREEN_SIZE], // Monochrome display
    keys: [bool; NUM_KEYS],      // Hex keypad state
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
            screen: [false; SCREEN_SIZE],
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
}

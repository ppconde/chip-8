pub mod chip8;
mod ram;

use minifb::{Window, WindowOptions};

fn main() {
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom("roms/glitchGhost.ch8").unwrap();
    let mut window = Window::new("CHIP-8 Emulator", 64, 32, WindowOptions::default()).unwrap();
}

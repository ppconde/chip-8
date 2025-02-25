pub mod chip8;
mod ram;

use minifb::{Window, WindowOptions};

fn run_emulator() {
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom("game.ch8").unwrap();

    let mut window = Window::new("CHIP-8 Emulator", 64, 32, WindowOptions::default()).unwrap();
}

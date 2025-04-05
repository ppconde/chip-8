pub mod chip8;
mod ram;

use minifb::{Key, Window, WindowOptions};
use rand::random;

fn main() {
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom("roms/glitchGhost.ch8").unwrap();
    let buffer = vec![0; 64 * 32]; // 1D buffer to store pixel colors

    let mut window = match Window::new(
        "CHIP-8 Emulator",
        64 * 10,
        32 * 10,
        WindowOptions::default(),
    ) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    window.set_title("Chip-8 Emulator!");
    match window.update_with_buffer(&buffer, 64, 32) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to update buffer {}", err);
        }
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let buffer: Vec<u32> = (0..64 * 32).map(|_| random::<u32>()).collect();
        window.update_with_buffer(&buffer, 64, 32).unwrap();
    }
}

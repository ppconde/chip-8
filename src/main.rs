pub mod chip8;
mod ram;

use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

fn main() {
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom("roms/pong2.ch8").unwrap();
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
        // Run one cycle of the CHIP 8 interpreter
        let frame_time = Duration::from_micros(1_000_000 / 700); // ~700Hz
        let start = Instant::now();
        chip8.run_cycle();
        let elapsed = start.elapsed();
        if elapsed < frame_time {
            std::thread::sleep(frame_time - elapsed);
        }

        let buffer = chip8.screen_buffer();
        window.update_with_buffer(&buffer, 64, 32).unwrap();
    }
}

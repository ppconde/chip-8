pub mod chip8;
mod ram;

use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};

fn main() {
    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom("roms/maze.ch8").unwrap();

    let mut window = match Window::new(
        "CHIP-8 Emulator",
        64 * 20,
        32 * 20,
        WindowOptions::default(),
    ) {
        Ok(win) => win,
        Err(err) => {
            println!("Unable to create window {}", err);
            return;
        }
    };

    window.set_title("Chip-8 Emulator!");

    // Initialize timing variables
    let mut last_cycle_time = Instant::now();
    let mut last_timer_update = Instant::now();
    let cycle_duration = Duration::from_micros(1_000_000 / 700); // ~700 Hz
    let timer_duration = Duration::from_micros(1_000_000 / 60); // 60 Hz

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        // Run CPU cycles at ~700Hz
        if now.duration_since(last_cycle_time) >= cycle_duration {
            println!(
                "Run cycle! last_cycle_time: {:?}, cycle_duration: {:?}",
                last_cycle_time, cycle_duration
            );

            chip8.run_cycle();
            last_cycle_time += cycle_duration;
        }

        // Update timers at 60Hz
        // if now.duration_since(last_timer_update) >= timer_duration {
        //     chip8.update_timers();
        //     last_timer_update = Instant::now();
        // }

        // Only update the display when needed
        if chip8.should_draw() {
            println!("Draw");
            // Get the screen buffer from chip8 and convert to format needed by minifb
            chip8.update_screen_buffer();
            let buffer = chip8.get_screen_buffer();
            window.update_with_buffer(&buffer, 64, 32).unwrap();
            chip8.reset_draw_flag();
        }

        // println!("Loop");
        // Add a small sleep to prevent hogging CPU
        // std::thread::sleep(Duration::from_millis(1));
    }
}

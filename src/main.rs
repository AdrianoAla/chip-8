use macroquad::prelude::*;
use std::fs;

#[macroquad::main("CHIP-8 EMULATOR")]
async fn main() {
    let rom_file = fs::read_to_string("rom.ch8").expect("No ROM provided!");
    println!("Hello, World!");
    loop {
        clear_background(WHITE);
        next_frame().await
    }
}

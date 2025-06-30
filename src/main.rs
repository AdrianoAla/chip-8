use macroquad::prelude::*;
use std::fs;

fn construct_u16(first: u8, second: u8) -> u16 {
    ((first as u16) << 8) + second as u16
}

struct Chip8 {
    memory: [u8; 4096],
    registers: [u8; 16],
    stack: [u16; 16],
    stack_pointer: usize,
    program_counter: usize,
    screen: [[bool; 32]; 64],
    rom: Vec<u8>,
}
impl Chip8 {
    pub fn new() -> Chip8 {
        Self {
            memory: [0; 4096],
            registers: [0; 16],
            stack: [0; 16],
            stack_pointer: 0,
            program_counter: 0x200,
            screen: [[false; 32]; 64],
            rom: vec![],
        }
    }

    pub fn load_rom(&mut self, name: String) {
        self.rom = fs::read(name).unwrap();
        for (index, item) in self.rom.iter().enumerate() {
            self.memory[index + 0x200] = *item;
        }
    }

    pub fn next_instruction(&mut self) {
        self.program_counter += 2;
    }

    pub fn get_instruction(&self) -> u16 {
        construct_u16(
            self.memory[self.program_counter],
            self.memory[self.program_counter + 1],
        )
    }

    pub fn goto(&mut self, addr: u16) {
        self.program_counter = addr as usize;
    }
}

#[macroquad::main("CHIP-8 EMULATOR")]
async fn main() {
    let mut console = Chip8::new();
    console.load_rom(String::from("rom.ch8"));

    let screen: [[bool; 32]; 64] = [[false; 32]; 64];

    let mut timer = 0.0;

    loop {
        timer += get_frame_time();

        // update

        if timer >= 1. / 2. {
            timer -= 1. / 2.;

            let instruction = console.get_instruction();
            println!("{:X}", instruction);

            match (instruction & 0xF000) >> 12 {
                0x0 => match instruction & 0x0FFF {
                    0x00EE => {
                        console.screen = [[false; 32]; 64];
                    }

                    0x00E0 => {
                        console.program_counter = console.stack[console.stack_pointer] as usize;
                        console.stack[console.stack_pointer] = 0;
                        console.stack_pointer -= 1;
                    }

                    _ => {}
                },
                0x1 => {
                    println!("GOTO {:X}", instruction & 0x0FFF);
                    console.goto(instruction & 0x0FFF);
                }
                0x2 => {
                    console.stack[console.stack_pointer] = (console.program_counter - 2) as u16;
                    console.stack_pointer += 1;
                    console.goto(instruction & 0x0FFF);
                }
                0x3 => {
                    let register = (instruction & 0x0F00) >> 8;
                    let value = instruction & 0x00FF;

                    if console.registers[register as usize] == value as u8 {
                        console.next_instruction();
                    }
                }
                0x4 => {
                    let register = (instruction & 0x0F00) >> 8;
                    let value = instruction & 0x00FF;

                    if console.registers[register as usize] != value as u8 {
                        console.next_instruction();
                    }
                }
                0x5 => {
                    let register_x = (instruction & 0x0F00) >> 8;
                    let register_y = (instruction & 0x00F0) >> 4;

                    if console.registers[register_x as usize]
                        != console.registers[register_y as usize]
                    {
                        console.next_instruction();
                    }
                }
                0x6 => {
                    let register = (instruction & 0x0F00) >> 8;
                    let value = instruction & 0x00FF;

                    console.registers[register as usize] = value as u8;
                }
                0x7 => {
                    let register = (instruction & 0x0F00) >> 8;
                    let value = instruction & 0x00FF;

                    console.registers[register as usize] += value as u8;
                }
                0x8 => {}
                0x9 => {}
                0xA => {}
                0xB => {}
                0xC => {}
                0xD => {}
                0xE => {}
                0xF => {}
                _ => {}
            }

            console.next_instruction();
        }

        // draw

        clear_background(WHITE);
        for x in 0..64 {
            for y in 0..32 {
                let mut colour = BLACK;
                if screen[x][y] {
                    colour = WHITE;
                }
                let size = 10.;
                draw_rectangle(x as f32 * size, y as f32 * size, size, size, colour);
            }
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}

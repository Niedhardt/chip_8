
mod font;
mod chip8;
mod drivers;

use std::env;
use drivers::{DisplayDriver, RomDriver};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::{Keycode, Scancode};
use std::time::Duration;
use sdl2::event;
use sdl2::rect::Point;
use crate::chip8::Chip8;

const CHIP8_MEMORY: usize = 4096;
const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;
pub fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut display_driver = DisplayDriver::new(&sdl_context);
    let args: Vec<String> = env::args().collect();
    let rom = RomDriver::new(&args[1]);
    let mut chip = Chip8::new();

    chip.insert_rom_data(rom);
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {
                    ..
                } | Event::KeyDown {scancode:Some(Scancode::W), ..} => break 'running,
                _ => {}
            }
        }


        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown => {
                    match Event::KeyDown {
                        event.Scancode::AcHome => ()aa
                    }
                }
                _ => {}
            }
        }
        /*JJ:WJJ
        let keys = event_pump.keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();*/


        let op_code = chip.get_opcode();
        chip.execute_opcodes(op_code);
        if chip.vram_changed {
            display_driver.fill_pixel(chip.vram);
        }

       // display_driver.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 /2));
    }

}
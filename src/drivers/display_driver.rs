use sdl2;
use sdl2::pixels;
use sdl2::rect::{Point, Rect};
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::{CHIP8_HEIGHT, CHIP8_WIDTH};

const SCALE_FACTOR: u32 = 20;
const CANVAS_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE_FACTOR;
const CANVAS_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE_FACTOR;


pub struct DisplayDriver {
   pub  canvas: Canvas<Window>,
}

impl DisplayDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window("rust-sdl2_gfx: draw line & FPSManager",
                CANVAS_WIDTH, CANVAS_HEIGHT)
                .position_centered()
                .opengl()
                .build()
                .unwrap();
            let mut canvas = window.into_canvas().build().unwrap();
            canvas.clear();
            canvas.present();

        Self {
            canvas,
        }
    }

    pub fn color(value: u8) -> pixels::Color {
        if value == 0 {
            return pixels::Color::RGB(0,0,0);
        }
        return pixels::Color::RGB(0,250,0);
    }

    pub fn fill_pixel(&mut self, vram: [[u8; 64]; 32])  {
        self.canvas.clear();
        for i in 0..vram.len() {
            for j in 0..vram[i].len() {
                let x = (i as u32) * SCALE_FACTOR ;
                let y = (j as u32) * SCALE_FACTOR ;
                if vram[i][j] == 1 {
                    self.canvas.set_draw_color(DisplayDriver::color(vram[i][j]));
                }

                self.canvas.fill_rect(Rect::new(x as i32, j as i32, SCALE_FACTOR, SCALE_FACTOR)).expect("TODO: panic message");
            }
        }
        self.canvas.present();
    }

}
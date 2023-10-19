
use crate::{CHIP8_HEIGHT, CHIP8_MEMORY, CHIP8_WIDTH};
use crate::drivers::{DisplayDriver, RomDriver};
use crate::font::FONT_SET;

#[derive(Debug)]
pub struct Chip8{
    pub vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],
    pub memory: [u8; CHIP8_MEMORY],
    pub stack: Vec<u16>,
    pub sound: u8,
    pub timer: u8,
    pub pc: u16,
    pub i: u16,
    pub v: [u8; 16],
    pub vram_changed: bool,
    //pub display_driver: DisplayDriver,
}

impl Chip8 {
    pub fn new() -> Self{
        let mut memory = [0u8; CHIP8_MEMORY];
        for i in 0..FONT_SET.len() {
            memory[i] = FONT_SET[i];
        }
        let stack = Vec::<u16>::with_capacity(16);
        Self{
            vram: [[0;  CHIP8_WIDTH]; CHIP8_HEIGHT],
            memory,
            stack,
            sound: 0,
            timer: 0,
            pc: 0x200,
            i: 0,
            v: [0u8; 16],
            vram_changed: false,
        }
    }
    pub fn insert_rom_data(&mut self, rom: RomDriver) {
        for i in 0..rom.rom.len() {
            if 0x200 + i + 1 < CHIP8_MEMORY {
                self.memory[0x200 + i] = rom.rom[i];
            } else {
                break;
            }

        }
    }

    pub fn get_opcode(&mut self) -> u16{
        self.vram_changed = false;
        let get = ((self.memory[self.pc as usize] as u16) << 8 ) | self.memory[(self.pc+1) as usize] as u16;
        self.pc += 2;

       return get;
    }
    pub fn draw() {

    }

    pub fn execute_opcodes(&mut self, opcode: u16) {
        let nibbles = (
            (opcode & 0xf000) >> 12 as u8,
            (opcode & 0x0f00) >> 8 as u8,
            (opcode & 0x00f0) >> 4 as u8,
            (opcode & 0x000f) as u8,
            );
        println!("PC: {:04x}", self.pc);
       // println!("{:x?}", nibbles);
        let nn = (opcode & 0x00ff) as u8;
        let nnn = (opcode & 0x0fff);
        let x = nibbles.1 as usize;
        let y = nibbles.2 as usize;
        let n = nibbles.3 as usize;
        match nibbles {
            (0x00, 0x00, 0x0e, 0x00) => self.op_00e0(),
            (0x00, 0x00, 0x0e, 0x0e) => () ,
            (0x01, _, _,_) => self.op_1nnn(nnn),
            (0x02, _,_,_) => self.op_2nnn(nnn),
            (0x03,_,_,_) => self.op_3xnn(x, nn),
            (0x04,_,_,_) => self.op_4xnn(x, nn),
            (0x05,_,_,0x00) => self.op_5xy0(x, y),
            (0x06,_,_,_) => self.op_6xnn(x, nn),
            (0x07,_,_,_) => self.op_7xnn(x, nn),
            (0x08,_,_,0x00) => self.op_8xy0(x, y),
            (0x08,_,_,0x01) => self.op_8xy1(x, y),
            (0x08,_,_,0x02) => self.op_8xy2(x, y),
            (0x08,_,_,0x03) => self.op_8xy3(x, y),
            (0x08,_,_,0x04) => self.op_8xy4(x, y),
            (0x08,_,_,0x05) => self.op_8xy5(x, y),
            (0x08,_,_,0x06) => self.op_8xy6(x, y),
            (0x08,_,_,0x07) => self.op_8xy7(x,y),
            (0x08,_,_,0x0e) => self.op_8xye(x,y),
            (0x09,_,_,0x00) => self.op_9xy0(x,y),
            (0x09,_,_,_) => (),
            (0x0a,_,_,_) => self.op_annn(nnn),
            (0x0b,_,_,_) => (),
            (0x0c,_,_,_) => (),
            (0x0d,_,_,_) => self.op_dxyn(x, y, n),
            (0x0e,_,0x09,0x0e) => (),
            (0x0e,_,0x0a,0x01) => (),
            (0x0f,_,0x00,0x07) => (),
            (0x0f,_,0x00,0x0a) => (),
            (0x0f,_,0x01,0x05) => (),
            (0x0f,_,0x01,0x08) => (),
            (0x0f,_,0x01,0x0e) => (),
            (0x0f,_,0x02,0x09) => (),
            (0x0f,_,0x03,0x03) => (),
            (0x0f,_,0x05,0x05) => (),
            (0x0f,_,0x06,0x05) => (),
            _ => ()//println!("undefined"),
        }
    }

    pub fn op_00e0(&mut self) {
        println!("clear screen");
        for i in 0..self.vram.len() {
            for j in 0..self.vram[i].len() {
                self.vram[i][j] = 0;
            }
        }
    }

    pub fn op_1nnn(&mut self, nnn: u16) {
        println!("jump: {}", nnn);

        self.pc = nnn;
    }

    pub fn op_2nnn(&mut self, nnn: u16) {
        self.stack.push(self.pc);
        self.pc = nnn;
    }

    pub fn op_00ee(&mut self) {
        let val = self.stack.pop().expect("Unable to pop");
        self.stack.push(val);
    }

    pub fn op_3xnn(&mut self, x: usize, nn: u8) {
        if self.v[x] == nn {
            self.pc += 2;
        }

    }

    pub fn op_4xnn(&mut self, x: usize, nn: u8) {
       if self.v[x] != nn {
           self.pc += 2;
       }
    }

    pub fn op_5xy0(&mut self, x: usize, y: usize) {
        if self.v[x] == self.v[y] {
            self.pc += 2;
        }
    }

    pub fn op_6xnn(&mut self, x: usize, nn: u8) {
        println!("vx = nn");
        self.v[x] = nn;
    }

    pub fn op_7xnn(&mut self, x: usize, nn: u8) {
        println!("vx = vx + nn");
        self.v[x] = self.v[x] + nn;
    }

    pub fn op_8xy0(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y];
    }
    pub fn op_8xy1(&mut self, x: usize, y:usize) {
        self.v[x] = self.v[x] | self.v[y];
    }

    pub fn op_8xy2(&mut self, x: usize, y:usize) {
        self.v[x] = self.v[x] & self.v[y];
    }

    pub fn op_8xy3(&mut self, x: usize, y:usize) {
        self.v[x] = self.v[x] ^ self.v[y];
    }

    pub fn op_8xy4(&mut self, x: usize, y: usize) {
        self.v[x] = self.v[y] + self.v[x];
        if self.v[x] > 255 {
            self.v[0x00f] = 1;
        } else {
            self.v[0x00f] = 0;
        }
    }

    pub fn op_8xy5(&mut self, x: usize, y:usize) {
        if self.v[x] > self.v[y] {
            self.v[0x00f] = 1;
        } else {
            self.v[0x00f] = 0;
        }
        self.v[x] = self.v[x] - self.v[y];
    }

    pub fn op_8xy7(&mut self, x: usize, y:usize) {
        if self.v[y] > self.v[x] {
            self.v[0x00f] = 1;
        } else {
            self.v[0x00f] = 0;
        }
        self.v[x] = self.v[y] - self.v[x];
    }

    pub fn op_8xy6(&mut self, x: usize, y: usize) {
        if self.v[x] & (1 << 0) == 1 {
            self.v[0x00f] = 1;
        } else {
            self.v[0x00f] = 0;
        }
        self.v[x] = self.v[x] >> 1;
    }

    pub fn op_8xye(&mut self, x: usize, y: usize) {
        if (self.v[x] & (1 << 7)) == 1 {
            self.v[0x00f] = 1;
        } else{
            self.v[0x00f] = 0;
        }
        self.v[x] = self.v[x] << 1;
    }

    pub fn op_9xy0(&mut self, x: usize, y: usize) {
        if self.v[x] != self.v[y] {
            self.pc += 2;
        }
    }

    pub fn op_annn(&mut self, nnn: u16) {
        println!("index = nnn");
        self.i = nnn;
    }

    pub fn op_dxyn(&mut self, x: usize, y: usize, n: usize) {
        println!("Display: ");
        let width = (self.v[x] as usize % CHIP8_WIDTH);
        let height = (self.v[y] as usize % CHIP8_HEIGHT);
        println!("hor: {}, ver: {}", width + n, height);

        for row in height..height+n {
           for col in width..width+8 {
               let check = (self.memory[self.i as usize + (row - height)] >> (7 - (col - width))) & 1;
               self.v[0x00f] |= check & self.vram[row][col];
               self.vram[row][col] ^= check;
           }
        }
        self.vram_changed = true;
    }

    pub fn op_ex9e(&mut self, x: usize) {
        if self.v[x] {

        }
    }
    pub fn fetch() {

    }


}



/*
const HEIGHT: i32 = 32;
const WIDTH: i32 = 64;
pub struct Display {
    pub size: Vec<Vec<bool>>,
    pub height: HEIGHT,
    pub width: WIDTH,
}*/
/*
impl Display {
    pub fn new(&mut self) -> Self {
        Self{
            size: vec![vec![false; ]; 32],
        }
    }

    pub fn add_pixel(&mut self) {

    }
}
*/


use std::fs::File;
use std::io::Read;

pub struct RomDriver {
    pub rom: [u8; 3584],
    pub size: usize,
}


impl RomDriver {
    pub fn new(filename: &str) -> Self {
        let mut f = File::open(filename).expect("file not found");
        let mut buffer = [0u8; 3584];

        let bytes_read = if let Ok(bytes_read) = f.read(&mut buffer) {
            //println!("bytes: {:x?}",  buffer);
            return Self {rom: buffer, size: bytes_read};
        } else {
           return  Self {rom: buffer, size: 0};
        };
    }
}
use std::arch::x86_64::_SIDD_MASKED_NEGATIVE_POLARITY;

use super::ram::Ram;

pub struct Chip8{
    ram: Ram,
    pub registers: [u8; 16],
    // Index register I: 16 b
    pub index: u16,
    // Stack of 16 b addresses
    pub stack: [u16; 64],
    // Current index of the top of the stack
    pub istack: usize,
    // Program counter (PC)
    pub pc: usize,
    // Delay timer (DT): 8 b
    pub dt: u8,
    // Sound timer (ST): 8 b
    pub st: u8,
    // Display memory
    pub display: [u8; 64*32],
    // Flag: update display (draw has been called)
    // pub display_update_flag: bool,
    // Flag: clear display
    // pub display_clear_flag: bool,
    // Flag: beep
    // pub beep_flag: bool,

}

impl Chip8{
    pub fn new() -> Chip8 {
        Chip8 {
            ram : Ram::new(),
            index :0, 
            istack : 0,
            pc : 0,
            st : 0,
            dt : 0,
            display : [0; 64*32],
            registers : [0; 16],
            stack : [0; 64],
        }
    }
    pub fn printmem(&self){
        self.ram.printmemashex();
    }

    pub fn load_rom(&mut self, data: &Vec<u8>){
        let offset:u16 = 0x200;
        for i in 0..data.len() {
            self.ram.write_byte(offset + (i as u16), data[i]);
        }
    }

    pub fn load_fonts(&mut self){
        let fonts: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for i in 0..fonts.len() {
            self.ram.write_byte(i as u16, fonts[i]);
        }
    }
}

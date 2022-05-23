pub fn test_ram(){
    println!("correctly opened module ram");
}

pub struct Ram {
    mem: [u8; 4096],
}

impl Ram{
    pub fn new() -> Ram{
        Ram {
            mem:[0; 4096],
        }
    }
    pub fn printmem(&self){
        print!("{:?} \n ram, of length {}", self.mem, self.mem.len());
    }
    pub fn printmemashex(&self){
        for i in 0..self.mem.len(){
            print!("{:#02X}, ",self.mem[i]);
        }
    }
    pub fn read_byte(&mut self, address: u16, value: u8){
    }
    pub fn write_byte(&mut self, address: u16, value: u8){
        self.mem[address as usize] = value;
    }
}

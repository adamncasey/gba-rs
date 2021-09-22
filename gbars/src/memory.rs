use log::{trace};

pub struct Memory {
    //todo actual memory
    mem: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { mem: vec![0; 1024] }
    }

    pub fn new_with_rom(rom: Vec<u8>) -> Memory {
        Memory { mem: rom }
    }

    pub fn get(&self, addr: u32) -> u32 {
        self.mem[addr as usize] as u32
    }

    pub fn get_word(&self, addr: u32) -> u32 {

        let result = ((self.mem[(addr+3) as usize] as u32) << 24)
            | ((self.mem[(addr + 2) as usize] as u32) << 16)
            | ((self.mem[(addr + 1) as usize] as u32) << 8)
            | self.mem[(addr + 0) as usize] as u32;
        trace!("get_word {} {}", addr, result);

        return result;
    }

    pub fn set(&mut self, addr: u32, val: u32) {
        self.mem[addr as usize] = val as u8;
    }
}

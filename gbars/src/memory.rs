pub struct Memory {
    //todo actual memory
    mem: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: vec![0; 1024],
        }
    }

    pub fn new_with_rom(rom: Vec<u8>) -> Memory {
        Memory {
            mem: rom
        }
    }

    pub fn get(&self, addr: u32) -> u32 {
        self.mem[addr as usize] as u32
    }

    pub fn set(&mut self, addr: u32, val: u32) {
        self.mem[addr as usize] = val as u8;
    }
}
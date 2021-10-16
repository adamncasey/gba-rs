use log::trace;

pub struct Memory {
    //todo actual memory
    bios: Vec<u8>,         // 16kb
    onboard_wram: Vec<u8>, // 256kb
    onchip_wram: Vec<u8>,  // 32kb
    rom: Vec<u8>,          // 32mb
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            bios: vec![0; 0x4000],
            onboard_wram: vec![0; 0x40000],
            onchip_wram: vec![0; 0x8000],
            rom: vec![0; 0x2000000],
        }
    }

    pub fn new_with_bios_and_rom(bios: Vec<u8>, rom: Vec<u8>) -> Memory {
        Memory {
            bios: bios,
            onboard_wram: vec![0; 0x40000],
            onchip_wram: vec![0; 0x8000],
            rom: rom,
        }
    }

    pub fn get_byte(&self, addr: u32) -> u8 {
        match addr {
            0x08000000..=0x08FFFFFF => {
                self.rom[addr as usize - 0x08000000]
            }
            _ => 0
        }
    }

    pub fn get_halfword(&self, addr: u32) -> u16 {
        0
    }

    pub fn get_word(&self, addr: u32) -> u32 {
        let result = ((self.get_byte(addr + 3) as u32) << 24)
            | ((self.get_byte(addr + 2) as u32) << 16)
            | ((self.get_byte(addr + 1) as u32) << 8)
            | self.get_byte(addr + 0) as u32;
        trace!("get_word {} {}", addr, result);

        return result;
    }
}

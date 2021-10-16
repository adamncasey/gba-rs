use gbars::{Cpu, Memory};
use structopt::StructOpt;

use std::fs;

#[derive(Debug, StructOpt)]
struct Opt {
    bios: String,
    rom: String,
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    let startpc = 0x10000;
    let decode_length = 100;

    let bios_data = fs::read(opt.bios).expect("Unable to read bios file");
    let rom_data = fs::read(opt.rom).expect("Unable to read rom file");

    let mut mem = Memory::new_with_bios_and_rom(bios_data, rom_data);

    let mut cpu = Cpu::new();
    cpu.r15 = startpc;

    loop {
        if cpu.r15 > (startpc + decode_length) {
            return;
        }

        cpu.cycle(&mut mem);
    }
}

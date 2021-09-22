use gbars::{Cpu, Memory};
use structopt::StructOpt;

use std::fs;

#[derive(Debug, StructOpt)]
struct Opt {
    rom: String,
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    let startpc = 0x10000;
    let decode_length = 100;

    let data = fs::read(opt.rom).expect("Unable to read file");

    let mut mem = Memory::new_with_rom(data);

    let mut cpu = Cpu::new();
    cpu.r15 = startpc;

    loop {
        if cpu.r15 > (startpc + decode_length) {
            return;
        }

        cpu.cycle(&mut mem);
    }
}

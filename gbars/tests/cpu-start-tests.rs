use gbars::{Cpu, Memory};

use object::{Object, ObjectSection};
use env_logger;

use std::fs;

#[test]
fn test_load_unaligned_test() {
    env_logger::init();

    let code = "tests/test-roms/load-unaligned.elf";
    let file = fs::read(code).unwrap();

    let obj_file = object::File::parse(&*file).unwrap();

    let text = obj_file.section_by_name(".text").unwrap();

    let mut cpu = Cpu::new();

    let mut mem = Memory::new_with_bios_and_rom(vec![0; 0x4000], text.data().unwrap().to_vec());

    let exit_addr = 0x12341234;

    cpu.r15 = 0x08000000;
    cpu.r14 = 0x12341234;

    let mut cycles = 0;

    loop {
        cpu.cycle(&mut mem);

        if cpu.r15 == exit_addr {
            assert_eq!(0, cpu.r0);
        }

        if cycles > 100 {
            panic!("Didn't reach exit addr in time");
        }

        cycles += 1;
    }
}


#[test]
fn test_combined_raw() {
    env_logger::init();

    let code = "tests/test-roms/combined.gba";
    let file = fs::read(code).unwrap();

    let mut cpu = Cpu::new();

    let mut mem = Memory::new_with_bios_and_rom(vec![0; 0x4000], file);

    let exit_addr = 0x12341234;

    cpu.r15 = 0x08000000;
    cpu.r14 = 0x12341234;

    let mut cycles = 0;

    loop {
        cpu.cycle(&mut mem);

        if cpu.r15 == exit_addr {
            assert_eq!(0, cpu.r0);
        }

        if cycles > 100 {
            panic!("Didn't reach exit addr in time");
        }

        cycles += 1;
    }
}


#[test]
fn test_simple_gba() {
    env_logger::init();

    let code = "tests/test-roms/simple.gba";
    let file = fs::read(code).unwrap();

    let mut cpu = Cpu::new();

    let mut mem = Memory::new_with_bios_and_rom(vec![0; 0x4000], file);

    let exit_addr = 0x12341234;

    cpu.r15 = 0x08000000;
    cpu.r14 = 0x12341234;

    let mut cycles = 0;

    loop {
        cpu.cycle(&mut mem);

        if cpu.r15 == exit_addr {
            assert_eq!(0, cpu.r0);
        }

        if cycles > 100 {
            panic!("Didn't reach exit addr in time");
        }

        cycles += 1;
    }
}

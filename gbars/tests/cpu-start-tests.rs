use gbars::{Cpu, Memory};

use env_logger;
use object::{Object, ObjectSection};

use std::fs;

fn init() {
    let _ = env_logger::builder()
        .is_test(true)
        .format_timestamp(None)
        .try_init();
}

#[test]
fn test_load_unaligned_test() {
    init();

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
fn test_combined_gba() {
    init();

    let code = "tests/test-roms/combined.gba";

    run_test(code, 0x08000000, 0, 100);
}

#[test]
fn test_simple_gba() {
    init();

    let code = "tests/test-roms/simple.gba";

    run_test(code, 0x08000000, 0, 100);
}

fn run_test(rom: &str, func_addr: u32, return_val: u32, max_cycles: u32) {
    let file = fs::read(rom).unwrap();

    let mut cpu = Cpu::new();

    let mut mem = Memory::new_with_bios_and_rom(vec![0; 0x4000], file);

    let exit_addr = 0x12341234;

    cpu.r0 = return_val + 1; // r0 shouldn't equal return_val
    cpu.r15 = func_addr;
    cpu.r14 = exit_addr;

    let mut cycles = 0;

    loop {
        log::info!("{}", cpu);
        cpu.cycle(&mut mem);

        if cpu.r15 == exit_addr {
            assert_eq!(return_val, cpu.r0);
            return;
        }

        if cycles > max_cycles {
            panic!("Didn't reach exit addr in time");
        }

        cycles += 1;
    }
}

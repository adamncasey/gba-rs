use crate::memory::Memory;
use crate::instruction::Instruction;

#[derive(Debug, PartialEq, Eq)]
pub struct Cpu {
    state: CpuState,
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,

    r13: u32, // sp
    r14: u32, // lr
    r15_pc: u32,
    cpsr: u32,

    // internal state
    fetched: u32, // Could be 16- or 32-bits (thumb/arm)
    decoded: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CpuState {
    Arm,
    Thumb,
}

// Registers as seen by the program
// These have no concept of banking
#[derive(Debug, PartialEq, Eq)]
pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,

    R13,
    R14,
    R15,
}

impl Cpu {
    pub fn cycle(&mut self, mem: &mut Memory) {
        let prev_fetched = self.fetched;
        let prev_decoded = self.decoded;

        // fetch
        self.fetched = mem.get(self.r15_pc);

        // decode
        self.decoded = prev_fetched;

        // execute
        // TODO Decode thumb
        let instr = Instruction::decode_arm(prev_decoded);

        // TODO actually execute
    }
}

use crate::instruction::Instruction;
use crate::memory::Memory;

use log::{info, debug};

#[derive(Debug, PartialEq, Eq)]
pub struct Cpu {
    state: CpuState,
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r12: u32,
    r13: u32,     // sp
    r14: u32,     // lr
    pub r15: u32, // pc
    cpsr: u32,

    // internal state
    fetched: Option<u32>, // Could be 16- or 32-bits (thumb/arm)
    decoded: Option<u32>,
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
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            state: CpuState::Arm,
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0, // sp
            r14: 0, // lr
            r15: 0, // pc
            cpsr: 0,

            // internal state
            fetched: None, // Could be 16- or 32-bits (thumb/arm)
            decoded: None,
        }
    }

    pub fn cycle(&mut self, mem: &mut Memory) {
        let prev_fetched = self.fetched;
        let prev_decoded = self.decoded;

        // fetch
        let new_fetch = mem.get_word(self.r15);
        self.fetched = Some(new_fetch);

        // decode
        self.decoded = prev_fetched;

        if let Some(prev_decoded) = prev_decoded {
            // execute
            // TODO Decode thumb
            let instr = Instruction::decode_arm(prev_decoded);
            info!("fetched {:8x}, exec {:?}", new_fetch, instr);
        } else {
            debug!("No execute");
        }

        // TODO actually execute

        self.r15 += 4;
    }
}

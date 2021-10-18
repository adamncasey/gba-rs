use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::execute;

use log::{info, debug};

#[derive(Debug, PartialEq, Eq)]
pub struct Cpu {
    state: CpuState,
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r4: u32,
    pub r5: u32,
    pub r6: u32,
    pub r7: u32,
    pub r8: u32,
    pub r9: u32,
    pub r10: u32,
    pub r11: u32,
    pub r12: u32,
    pub r13: u32, // sp
    pub r14: u32, // lr
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
        let initial_cpsr = 0b11010011; // I=1 F=1 T=0 , M=supervisor

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
            cpsr: initial_cpsr,

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
            info!("fetched {:8x}, exec {:8x} {:?}", new_fetch, prev_decoded, instr);

            execute::execute(self, instr);
        } else {
            debug!("No execute");
        }
        
        if let Some(fetched) = self.fetched {
            // We didn't jump
            self.r15 += 4;
        }
    }

    pub fn flush_pipeline(&mut self) {
        self.fetched = None;
        self.decoded = None;
    }

    pub fn get_register(&self, reg: Register) -> u32{
        match reg {
            Register::R0 => self.r0,
            Register::R1 => self.r1,
            Register::R2 => self.r2,
            Register::R3 => self.r3,
            Register::R4 => self.r4,
            Register::R5 => self.r5,
            Register::R6 => self.r6,
            Register::R7 => self.r7,
            Register::R8 => self.r8,
            Register::R9 => self.r9,
            Register::R10 => self.r10,
            Register::R11 => self.r11,
            Register::R12 => self.r12,
            Register::R13 => self.r13,
            Register::R14 => self.r14,
            Register::R15 => self.r15,
        }
    }
}

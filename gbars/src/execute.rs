use crate::instruction::{Instruction, InstructionOp, Branch};
use crate::cpu::Cpu;

use log;

pub fn execute(cpu: &mut Cpu, instr: Instruction)
{
    // TODO Handle instr.condition
    match instr.instruction {
        InstructionOp::Branch { branch } => {
            execute_branch(cpu, branch);
        },
        _ => {
            log::info!("");
        }
    }
}

fn execute_branch(cpu: &mut Cpu, branch: Branch) {
    match branch {
        Branch::Offset {offset, link} => {
            let newpc = offset << 2; // TODO shift?

            if link {
                // TODO probably r15 isn't right due to pipelining
                cpu.r14 = cpu.r15;
            }

            // TODO Some way of passing what the pc value was for the current instruction..
            cpu.r15 = (cpu.r15 - 4) + newpc;

            cpu.flush_pipeline();
        }
        Branch::Exchange { register } => {
            let val: u32 = cpu.get_register(register);

            if val & 0b1 != 0 {
                // TODO Switch to thumb
            }

            cpu.r15 = val & (!0b1);
        }
    }
}
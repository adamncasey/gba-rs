use crate::cpu::Cpu;
use crate::instruction::{Branch, Instruction, InstructionOp};

fn sign_extend_24(num: u32) -> i32 {
    let sign_extend = num & (0b1 << 23) != 0;

    if sign_extend {
        ((0b11111111 << 24) | num) as i32
    } else {
        num as i32
    }
}

use log;

pub fn execute(cpu: &mut Cpu, instr: Instruction) {
    // TODO Handle instr.condition
    match instr.instruction {
        InstructionOp::Branch { branch } => {
            execute_branch(cpu, branch);
        }
        _ => {
            log::info!("");
        }
    }
}

fn execute_branch(cpu: &mut Cpu, branch: Branch) {
    match branch {
        Branch::Offset { offset, link } => {
            // offset is a 24bit signed two's complement number

            let signed_extended = sign_extend_24(offset);
            let backwards = offset & (0b1 << 23) != 0;

            let newpc = signed_extended << 2;

            if link {
                // TODO Need to know if we're thumb or not
                cpu.r14 = cpu.r15 - 4;
            }

            let newpc = if backwards {
                cpu.r15 - newpc.wrapping_abs() as u32
            } else {
                cpu.r15 + newpc.wrapping_abs() as u32
            };

            log::info!("Branch to {:8x}", newpc);

            cpu.r15 = newpc;

            cpu.flush_pipeline();
        }
        Branch::Exchange { register } => {
            let val: u32 = cpu.get_register(register);

            let thumb = val & 0b1 != 0;

            cpu.r15 = val & (!0b1);
            
            log::info!("Branch ({}) to {:8x}", if thumb {"Thumb"} else {"Arm"}, cpu.r15);
            
            cpu.flush_pipeline();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sign_extend() {
        let no_extend: u32 = 0b00111;

        assert_eq!(no_extend as i32, sign_extend_24(no_extend));

        let extend = 0b0000000_11111111_11111111_11111000;

        assert_eq!(
            -8i32,
            sign_extend_24(extend)
        );
    }
}

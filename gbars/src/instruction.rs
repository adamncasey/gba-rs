use crate::cpu::Register;

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    cond: Condition,
    instr: InstructionOp,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Condition {
    Equal,
    NotEqual,
    UnsignedGe,
    UnsignedLt,
    Negative,
    Positive,
    Oveflow,
    NoOverflow,
    UnsignedGt,
    UnsignedLe,
    Ge,
    Lt,
    Gt,
    Le,
    Always,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InstructionOp {
    DataProcessing {
        opcode: DataProcessingOpCode,
        dest: Register,
        operand1: Register,
        operand2: Operand,
        alter_condition: bool,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DataProcessingOpCode {
    And,
    Eor,
    Sub,
    Rsb,
    Add,
    Adc,
    Sbc,
    Rsc,
    Tst,
    Teq,
    Cmp,
    Cmn,
    Orr,
    Mov,
    Bic,
    Mvn,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operand {
    Register {
        shift: u8,
        register: Register,
    },
    Immediate {
        rotate: u8, // 4 bits
        value: u8,
    }
}

impl Instruction {
    pub fn decode_arm(instr: u32) -> Instruction {
        const CONDITION_MASK: u32 = 0b00001111111111111111111111111111;

        Instruction {
            cond: read_condition((instr & !CONDITION_MASK) >> 28),
            instr: read_instruction_op(instr & CONDITION_MASK)
        }
    }
}

fn read_condition(cond: u32) -> Condition {
    match cond {
        0 => Condition::Equal,
        1 => Condition::NotEqual,
        2 => Condition::UnsignedGe,
        3 => Condition::UnsignedLt,
        4 => Condition::Negative,
        5 => Condition::Positive,
        6 => Condition::Oveflow,
        7 => Condition::NoOverflow,
        8 => Condition::UnsignedGt,
        9 => Condition::UnsignedLe,
        10 => Condition::Ge,
        11 => Condition::Lt,
        12 => Condition::Gt,
        13 => Condition::Le,
        14 => Condition::Always,
        _ => panic!("Unimplemented condition: {:b}", cond)
    }
}

fn read_instruction_op(op: u32) -> InstructionOp {
    let b_27_26 = op >> 26;

    match b_27_26 {
        0b00 => {
            // look at bit 7..=4
            // Multiply, Swap, halfword transfer, BX, Data Processing
        },
        0b01 => {
            // Single Data Transfer / Undefined
        },
        0b10 => {
            // if b25 branch
            // else block data transfer
        },
        0b11 => {
            // Coprocessor / Software interrupt
        },
        _ => unimplemented!("bad read_instruction_op input: {} {}", op, b_27_26)
    }

    InstructionOp::DataProcessing {
        opcode: DataProcessingOpCode::And,
        dest: Register::R5,
        operand1: Register::R5,
        operand2: Operand::Immediate {
            rotate: 0,
            value: 3,
        },
        alter_condition: false,
    }
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_add_decode() {
    let op = 0xe0833002;

    let instr = Instruction::decode_arm(op);

    let expected = Instruction {
        cond: Condition::Always,
        instr: InstructionOp::DataProcessing {
            opcode: DataProcessingOpCode::Add,
            dest: Register::R3,
            operand1: Register::R3,
            operand2: Operand::Register {
                shift: 0,
                register: Register::R2,
            },
            alter_condition: false,
        },
    };

    assert_eq!(instr, expected);
}
}
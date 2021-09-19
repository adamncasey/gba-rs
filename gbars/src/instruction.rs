use crate::cpu::Register;
use bitvec::prelude::*;

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
    let bits = op.view_bits::<Lsb0>();
    let b_27_26 = &bits[26..=27];

    match b_27_26 {
        bits![0, 0] => {
            if bits[25] {
                // decode Data Processing with Immediate operand2
                return decode_data_processing(true, &bits[0..=24])
            }
            else {
                if bits[4] {
                    if bits[7] {
                        if bits[5] || bits[6] {
                            if bits[22] {
                                // Halfword transfer immediate offset
                            }
                            else {
                                // Halfword transfer register offset
                            }
                        }
                        else {
                            if bits[24] {
                                // Swap
                            }
                            else {
                                if bits[23] {
                                    // Multiply Long
                                }
                                else {
                                    // Multiply
                                }
                            }
                        }
                        // Multiply, Swap, halfword transfer
                    }
                    else {
                        // BX
                    }
                }
                else {
                    // Data Processing with Register operand2
                }
            }
        },
        bits![0, 1] => {
            if bits[25] && bits[4] {
                // Undefined instruction
            }
            else {
                // Single Data Transfer
            }
        },
        bits![1, 0] => {
            if bits[25] {
                // Branch
            }
            else {
                // Block data transfer
            }
        },
        bits![1, 1] => {
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

fn decode_data_processing(immediate: bool, bits: &BitSlice<Lsb0, u32>) -> InstructionOp {

    let alter_condition = bits[20];
    let opcode = read_dataprocessing_opcode(&bits[21..=24]);

    InstructionOp::DataProcessing {
        opcode,
        dest: Register::R5,
        operand1: Register::R5,
        operand2: Operand::Immediate {
            rotate: 0,
            value: 3,
        },
        alter_condition,
    }
}

fn read_dataprocessing_opcode(bits: &BitSlice<Lsb0, u32>) -> DataProcessingOpCode
{
    match bits {
        bits![0,0,0,0] => DataProcessingOpCode::And,
        bits![0,0,0,1] => DataProcessingOpCode::Eor,
        bits![0,0,1,0] => DataProcessingOpCode::Sub,
        bits![0,0,1,1] => DataProcessingOpCode::Rsb,
        bits![0,1,0,0] => DataProcessingOpCode::Add,
        bits![0,1,0,1] => DataProcessingOpCode::Adc,
        bits![0,1,1,0] => DataProcessingOpCode::Sbc,
        bits![0,1,1,1] => DataProcessingOpCode::Rsc,
        bits![1,0,0,0] => DataProcessingOpCode::Tst,
        bits![1,0,0,1] => DataProcessingOpCode::Teq,
        bits![1,0,1,0] => DataProcessingOpCode::Cmp,
        bits![1,0,1,1] => DataProcessingOpCode::Cmn,
        bits![1,1,0,0] => DataProcessingOpCode::Orr,
        bits![1,1,0,1] => DataProcessingOpCode::Mov,
        bits![1,1,1,0] => DataProcessingOpCode::Bic,
        bitvec![Lsb0, u32; 1,1,1,1] => DataProcessingOpCode::Mvn,
        _ => panic!("Bad input to read_dataprocessing_opecode: {:?} ", bits.to_bitvec()),
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
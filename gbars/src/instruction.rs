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
    let b_27_26 = op >> 26;

    match b_27_26 {
        0b00 => {
            if bits[25] {
                // decode Data Processing with Immediate operand2
                return decode_data_processing(true, op);
            }
            else {
                if bits[4] {
                    if bits[7] {
                        if bits[5] || bits[6] {
                            if bits[22] {
                                unimplemented!("Halfword transfer immediate offset");
                            }
                            else {
                                unimplemented!("Halfword transfer register offset");
                            }
                        }
                        else {
                            if bits[24] {
                                unimplemented!("Swap");
                            }
                            else {
                                if bits[23] {
                                    unimplemented!("Multiply Long");
                                }
                                else {
                                    unimplemented!("Multiply");
                                }
                            }
                        }
                        unimplemented!("Multiply, Swap, halfword transfer");
                    }
                    else {
                        unimplemented!("BX");
                    }
                }
                else {
                    return decode_data_processing(false, op);
                }
            }
        },
        0b01 => {
            if bits[25] && bits[4] {
                unimplemented!("Undefined instruction");
            }
            else {
                unimplemented!("Single Data Transfer");
            }
        },
        0b10 => {
            if bits[25] {
                unimplemented!("Branch");
            }
            else {
                unimplemented!("Block data transfer");
            }
        },
        0b11 => {
            unimplemented!("Coprocessor / Software interrupt");
        },
        _ => unimplemented!("bad read_instruction_op input: {} {}", op, b_27_26)
    }
}

fn decode_data_processing(immediate: bool, bits: u32) -> InstructionOp {
    let alter_condition = ((bits >> 20) & 0b1) != 0;
    let opcode = read_dataprocessing_opcode(((bits >> 21) & 0b1111) as u8);

    let rd = read_register(((bits >> 12) & 0b1111) as u8);
    let rn = read_register(((bits >> 16) & 0b1111) as u8);

    let operand2 = if immediate {
        Operand::Immediate {
            rotate: ((bits >> 8) & 0b1111) as u8,
            value: (bits & 0b11111111) as u8,
        }
    } else {
        let rm = read_register((bits & 0b1111) as u8);

        let shift = ((bits >> 4) & 0b11111111) as u8;

        Operand::Register {
            shift,
            register: rm
        }
    };

    InstructionOp::DataProcessing {
        opcode,
        dest: rd,
        operand1: rn,
        operand2,
        alter_condition,
    }
}

fn read_dataprocessing_opcode(bits: u8) -> DataProcessingOpCode
{
    match bits {
        0b0000 => DataProcessingOpCode::And,
        0b0001 => DataProcessingOpCode::Eor,
        0b0010 => DataProcessingOpCode::Sub,
        0b0011 => DataProcessingOpCode::Rsb,
        0b0100 => DataProcessingOpCode::Add,
        0b0101 => DataProcessingOpCode::Adc,
        0b0110 => DataProcessingOpCode::Sbc,
        0b0111 => DataProcessingOpCode::Rsc,
        0b1000 => DataProcessingOpCode::Tst,
        0b1001 => DataProcessingOpCode::Teq,
        0b1010 => DataProcessingOpCode::Cmp,
        0b1011 => DataProcessingOpCode::Cmn,
        0b1100 => DataProcessingOpCode::Orr,
        0b1101 => DataProcessingOpCode::Mov,
        0b1110 => DataProcessingOpCode::Bic,
        0b1111 => DataProcessingOpCode::Mvn,
        _ => panic!("Bad input to read_dataprocessing_opecode: {:?} ", bits),
    }
}

fn read_register(bits: u8) -> Register {
    match bits {
        0b0000 => Register::R0,
        0b0001 => Register::R1,
        0b0010 => Register::R2,
        0b0011 => Register::R3,
        0b0100 => Register::R4,
        0b0101 => Register::R5,
        /*0b0110 => Register::R6,
        0b0111 => Register::R7,
        0b1000 => Register::R8,
        0b1001 => Register::R9,
        0b1010 => Register::R10,
        0b1011 => Register::R11,
        0b1100 => Register::R12,*/
        0b1101 => Register::R13,
        0b1110 => Register::R14,
        0b1111 => Register::R15,
        _ => panic!("Bad input to read_register: {:?} ", bits),
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

#[test]
fn test_sub_decode() {
    let op = 0xe24dd014;

    let instr = Instruction::decode_arm(op);

    let expected = Instruction {
        cond: Condition::Always,
        instr: InstructionOp::DataProcessing {
            opcode: DataProcessingOpCode::Sub,
            dest: Register::R13,
            operand1: Register::R13,
            operand2: Operand::Immediate {
                rotate: 0,
                value: 20,
            },
            alter_condition: false,
        },
    };

    assert_eq!(instr, expected);
}

#[test]
fn test_mov_decode() {
    let op = 0xe3a03005;

    let instr = Instruction::decode_arm(op);

    let expected = Instruction {
        cond: Condition::Always,
        instr: InstructionOp::DataProcessing {
            opcode: DataProcessingOpCode::Mov,
            dest: Register::R3,
            operand1: Register::R0,
            operand2: Operand::Immediate {
                rotate: 0,
                value: 5,
            },
            alter_condition: false,
        },
    };

    assert_eq!(instr, expected);
}

#[test]
fn test_push_decode() {
    let op = 0xe52db004;

    let instr = Instruction::decode_arm(op);

    let expected = Instruction {
        cond: Condition::Always,
        instr: InstructionOp::DataProcessing {
            opcode: DataProcessingOpCode::Mov,
            dest: Register::R3,
            operand1: Register::R0,
            operand2: Operand::Immediate {
                rotate: 0,
                value: 5,
            },
            alter_condition: false,
        },
    };

    assert_eq!(instr, expected);
}
}
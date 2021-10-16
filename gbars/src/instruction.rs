use crate::cpu::Register;
use bitvec::prelude::*;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    pub condition: Condition,
    pub instruction: InstructionOp,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum InstructionOp {
    DataProcessing {
        opcode: DataProcessingOpCode,
        dest: Register,
        operand1: Register,
        operand2: Operand,
        alter_condition: bool,
    },
    Multiply {
        dest: Register,
        operand1: Register,
        operand2: Register,
        accumulate: bool,
        acc_operand: Register,
        alter_condition: bool,
    },
    MultiplyLong {
        dest_high: Register,
        dest_low: Register,
        operand1: Register,
        operand2: Register,
        accumulate: bool,
        signed: bool,
        alter_condition: bool,
    },
    Swap {
        source: Register,
        dest: Register,
        base: Register,
        byte: bool,
    },
    Branch {
        branch: Branch,
    },
    SingleDataTransfer {
        base: Register,
        source_dest: Register,
        load: bool, // false = store
        write_back: bool,
        write_byte: bool, // false = word
        add_offset: bool, // false = subtract offset
        pre_index: bool,  // false = post, Add offset after transfer
        offset: Offset,
    },
    BlockDataTransfer {
        base: Register,
        load: bool, // false = store
        write_back: bool,
        force_psr: bool,
        add_offset: bool,
        pre_index: bool,
        register_list: Vec<Register>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Branch {
    Exchange {
        register: Register,
    },
    Offset {
        offset: u32, // 24 bits
        link: bool,  // if true, write next instr addr to link/r14 register
    },
}

#[derive(Debug, PartialEq)]
pub enum Offset {
    Immediate {
        offset: u16, // 12 bits
    },
    Register {
        shift: u8,
        register: Register,
    },
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Operand {
    Register {
        shift: u8,
        register: Register,
    },
    Immediate {
        rotate: u8, // 4 bits
        value: u8,
    },
}

impl Instruction {
    pub fn decode_arm(instr: u32) -> Instruction {
        const CONDITION_MASK: u32 = 0b00001111_11111111_11111111_11111111;

        Instruction {
            condition: read_condition((instr & !CONDITION_MASK) >> 28),
            instruction: read_instruction_op(instr & CONDITION_MASK),
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
        _ => panic!("Unimplemented condition: {:b}", cond),
    }
}

fn read_instruction_op(op: u32) -> InstructionOp {
    let bits = op.view_bits::<Lsb0>();
    let b_27_26 = op >> 26;

    match b_27_26 {
        0b00 => {
            if bits[25] {
                // decode Data Processing with Immediate operand2
                decode_data_processing(true, op)
            } else {
                if bits[4] {
                    if bits[7] {
                        if bits[5] || bits[6] {
                            if bits[22] {
                                unimplemented!("Halfword transfer immediate offset");
                            } else {
                                unimplemented!("Halfword transfer register offset");
                            }
                        } else {
                            if bits[24] {
                                decode_swap(op)
                            } else {
                                if bits[23] {
                                    decode_multiply_long(op)
                                } else {
                                    decode_multiply(op)
                                }
                            }
                        }
                    } else {
                        decode_branch_exchange(op)
                    }
                } else {
                    return decode_data_processing(false, op);
                }
            }
        }
        0b01 => {
            if bits[25] && bits[4] {
                unimplemented!("Undefined instruction");
            } else {
                decode_single_data_transfer(op)
            }
        }
        0b10 => {
            if bits[25] {
                decode_branch(op)
            } else {
                decode_block_data_transfer(op)
            }
        }
        0b11 => {
            unimplemented!("Coprocessor / Software interrupt");
        }
        _ => unimplemented!("bad read_instruction_op input: {} {}", op, b_27_26),
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
            register: rm,
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

fn decode_single_data_transfer(bits: u32) -> InstructionOp {
    let offset = if ((bits >> 25) & 0b1) != 0 {
        Offset::Register {
            shift: ((bits >> 4) & 0b11111111) as u8,
            register: read_register((bits & 0b1111) as u8),
        }
    } else {
        Offset::Immediate {
            offset: (bits & 0b1111_11111111) as u16,
        }
    };

    InstructionOp::SingleDataTransfer {
        base: read_register(((bits >> 16) & 0b1111) as u8),
        source_dest: read_register(((bits >> 12) & 0b1111) as u8),
        load: ((bits >> 20) & 0b1) != 0,
        write_back: ((bits >> 21) & 0b1) != 0,
        write_byte: ((bits >> 22) & 0b1) != 0, // false = word
        add_offset: ((bits >> 23) & 0b1) != 0, // false = subtract offset
        pre_index: ((bits >> 24) & 0b1) != 0,  // false = post, Add offset after transfer
        offset,
    }
}

fn decode_branch_exchange(bits: u32) -> InstructionOp {
    InstructionOp::Branch {
        branch: Branch::Exchange {
            register: read_register((bits & 0b1111) as u8),
        },
    }
}

fn decode_branch(bits: u32) -> InstructionOp {
    InstructionOp::Branch {
        branch: Branch::Offset {
            offset: bits & 0b11111111_11111111_11111111,
            link: ((bits >> 24) & 0b1) != 0,
        },
    }
}

fn decode_multiply(bits: u32) -> InstructionOp {
    InstructionOp::Multiply {
        dest: read_register(((bits >> 16) & 0b1111) as u8),
        operand1: read_register((bits & 0b1111) as u8),
        operand2: read_register(((bits >> 8) & 0b1111) as u8),
        accumulate: ((bits >> 21) & 0b1) != 0,
        acc_operand: read_register(((bits >> 12) & 0b1111) as u8),
        alter_condition: ((bits >> 20) & 0b1) != 0,
    }
}

fn decode_multiply_long(bits: u32) -> InstructionOp {
    InstructionOp::MultiplyLong {
        dest_high: read_register(((bits >> 16) & 0b1111) as u8),
        dest_low: read_register(((bits >> 12) & 0b1111) as u8),
        operand1: read_register((bits & 0b1111) as u8),
        operand2: read_register(((bits >> 8) & 0b1111) as u8),
        accumulate: ((bits >> 21) & 0b1) != 0,
        signed: ((bits >> 22) & 0b1) != 0,
        alter_condition: ((bits >> 20) & 0b1) != 0,
    }
}

fn decode_swap(bits: u32) -> InstructionOp {
    InstructionOp::Swap {
        source: read_register((bits & 0b1111) as u8),
        dest: read_register(((bits >> 12) & 0b1111) as u8),
        base: read_register(((bits >> 16) & 0b1111) as u8),
        byte: ((bits >> 22) & 0b1) != 0,
    }
}

fn decode_block_data_transfer(bits: u32) -> InstructionOp {
    let mut register_list = Vec::new();

    for reg in 0..=15 {
        let mask = 0b1 << reg;

        if bits & mask != 0 {
            register_list.push(read_register(reg));
        }
    }

    InstructionOp::BlockDataTransfer {
        base: read_register(((bits >> 16) & 0b1111) as u8),
        load: ((bits >> 20) & 0b1) != 0,
        write_back: ((bits >> 21) & 0b1) != 0,
        force_psr: ((bits >> 22) & 0b1) != 0,
        add_offset: ((bits >> 23) & 0b1) != 0,
        pre_index: ((bits >> 24) & 0b1) != 0,
        register_list,
    }
}

fn read_dataprocessing_opcode(bits: u8) -> DataProcessingOpCode {
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
        0b0110 => Register::R6,
        0b0111 => Register::R7,
        0b1000 => Register::R8,
        0b1001 => Register::R9,
        0b1010 => Register::R10,
        0b1011 => Register::R11,
        0b1100 => Register::R12,
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
            condition: Condition::Always,
            instruction: InstructionOp::DataProcessing {
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
            condition: Condition::Always,
            instruction: InstructionOp::DataProcessing {
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
            condition: Condition::Always,
            instruction: InstructionOp::DataProcessing {
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
            condition: Condition::Always,
            instruction: InstructionOp::SingleDataTransfer {
                base: Register::R13,
                source_dest: Register::R11,
                load: false,
                write_back: true,
                write_byte: false,
                add_offset: false,
                pre_index: true,
                offset: Offset::Immediate { offset: 4 },
            },
        };

        assert_eq!(instr, expected);
    }

    #[test]
    fn test_str_decode() {
        let op = 0xe50b3008;

        let instr = Instruction::decode_arm(op);

        let expected = Instruction {
            condition: Condition::Always,
            instruction: InstructionOp::SingleDataTransfer {
                base: Register::R11,
                source_dest: Register::R3,
                load: false,
                write_back: false,
                write_byte: false,
                add_offset: false,
                pre_index: true,
                offset: Offset::Immediate { offset: 8 },
            },
        };

        assert_eq!(instr, expected);
    }

    #[test]
    fn test_ldr_decode() {
        let op = 0xe51b2008;

        let instr = Instruction::decode_arm(op);

        let expected = Instruction {
            condition: Condition::Always,
            instruction: InstructionOp::SingleDataTransfer {
                base: Register::R11,
                source_dest: Register::R2,
                load: true,
                write_back: false,
                write_byte: false,
                add_offset: false,
                pre_index: true,
                offset: Offset::Immediate { offset: 8 },
            },
        };

        assert_eq!(instr, expected);
    }

    #[test]
    fn test_pop_decode() {
        let op = 0xe49db004;

        let instr = Instruction::decode_arm(op);

        let expected = Instruction {
            condition: Condition::Always,
            instruction: InstructionOp::SingleDataTransfer {
                base: Register::R13,
                source_dest: Register::R11,
                load: true,
                write_back: false,
                write_byte: false,
                add_offset: true,
                pre_index: false,
                offset: Offset::Immediate { offset: 4 },
            },
        };

        assert_eq!(instr, expected);
    }

    #[test]
    fn test_bx_decode() {
        let op = 0xe12fff1e;

        let instr = Instruction::decode_arm(op);

        let expected = Instruction {
            condition: Condition::Always,
            instruction: InstructionOp::Branch {
                branch: Branch::Exchange {
                    register: Register::R14,
                },
            },
        };

        assert_eq!(instr, expected);
    }

    #[test]
    fn test_b_decode() {
        let op = 0xea000032;

        let instr = Instruction::decode_arm(op);

        let expected = Instruction {
            condition: Condition::Always,
            instruction: InstructionOp::Branch {
                branch: Branch::Offset {
                    offset: 50,
                    link: false,
                },
            },
        };

        assert_eq!(instr, expected);
    }

    #[test]
    fn test_mul_decode() {
        let op = 0xe0030392;

        let instr = Instruction::decode_arm(op);

        let expected = Instruction {
            condition: Condition::Always,
            instruction: InstructionOp::Multiply {
                dest: Register::R3,
                operand1: Register::R2,
                operand2: Register::R3,
                accumulate: false,
                acc_operand: Register::R0,
                alter_condition: false,
            },
        };

        assert_eq!(instr, expected);
    }

    #[test]
    fn test_block_data_transfer_push_decode() {
        let op = 0xe92d4800;

        let instr = Instruction::decode_arm(op);

        let expected = Instruction {
            condition: Condition::Always,
            instruction: InstructionOp::BlockDataTransfer {
                base: Register::R13,
                load: false,
                write_back: true,
                force_psr: false,
                add_offset: false,
                pre_index: true,
                register_list: vec![Register::R11, Register::R14],
            },
        };

        assert_eq!(instr, expected);
    }
}

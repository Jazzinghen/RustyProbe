use crate::utils::data_address::{
    parse_address, AddresingMode, AsmInstruction, DataMode, Register,
};
use std::fmt;
use std::ops::{Shl, Shr};

// Double operands instructions (Format I)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwoOp {
    Mov,
    Add,
    Addc,
    Sub,
    Subc,
    Cmp,
    Dadd,
    Bit,
    Bic,
    Bis,
    Xor,
    And,
}

impl TryFrom<u16> for TwoOp {
    type Error = &'static str;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let op_data: u16 = value.shr(12);
        if op_data == 0b0001 {
            return Err("the provided word is not a two operand op code");
        }
        match op_data {
            0b0100 => Ok(Self::Mov),
            0b0101 => Ok(Self::Add),
            0b0110 => Ok(Self::Addc),
            0b0111 => Ok(Self::Subc),
            0b1000 => Ok(Self::Sub),
            0b1001 => Ok(Self::Cmp),
            0b1010 => Ok(Self::Dadd),
            0b1011 => Ok(Self::Bit),
            0b1100 => Ok(Self::Bic),
            0b1101 => Ok(Self::Bis),
            0b1110 => Ok(Self::Xor),
            0b1111 => Ok(Self::And),
            _ => Err("inexisting operation"),
        }
    }
}

impl From<TwoOp> for u16 {
    fn from(val: TwoOp) -> Self {
        let op_bits = match val {
            TwoOp::Mov => 0b0100u16,
            TwoOp::Add => 0b0101u16,
            TwoOp::Addc => 0b0110u16,
            TwoOp::Subc => 0b0111u16,
            TwoOp::Sub => 0b1000u16,
            TwoOp::Cmp => 0b1001u16,
            TwoOp::Dadd => 0b1010u16,
            TwoOp::Bit => 0b1011u16,
            TwoOp::Bic => 0b1100u16,
            TwoOp::Bis => 0b1101u16,
            TwoOp::Xor => 0b1110u16,
            TwoOp::And => 0b1111u16,
        };
        op_bits.shl(12)
    }
}

impl From<TwoOp> for String {
    fn from(val: TwoOp) -> Self {
        match val {
            TwoOp::Mov => "mov",
            TwoOp::Add => "add",
            TwoOp::Addc => "addc",
            TwoOp::Subc => "subc",
            TwoOp::Sub => "sub",
            TwoOp::Cmp => "cmp",
            TwoOp::Dadd => "dadd",
            TwoOp::Bit => "bit",
            TwoOp::Bic => "bic",
            TwoOp::Bis => "bis",
            TwoOp::Xor => "xor",
            TwoOp::And => "and",
        }
        .to_string()
    }
}

impl fmt::Display for TwoOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

#[derive(Debug)]
pub enum EmulatedOp {
    Ret,
    Clrc,
    Setc,
    Clrz,
    Setz,
    Clrn,
    Setn,
    Dint,
    Eint,
    Nop,
    Br,
    Pop,
    Rla,
    Rlc,
    Inv,
    Clr,
    Tst,
    Dec,
    Decd,
    Inc,
    Incd,
    Adc,
    Dadc,
    Sbc,
}

#[derive(Debug)]
pub struct TwoOpInstruction {
    operation: TwoOp,
    source: AddresingMode,
    destination: AddresingMode,
    mode: DataMode,
}

impl AsmInstruction for TwoOpInstruction {}

impl TwoOpInstruction {
    pub fn new(raw_words: &[u16; 3]) -> (Self, u8) {
        let operation = TwoOp::try_from(raw_words[0]).unwrap();
        let (source, destination, extra_words) = Self::parse_address(raw_words);
        let mode = DataMode::from(raw_words[0]);

        (
            Self {
                operation,
                source,
                destination,
                mode,
            },
            extra_words,
        )
    }

    fn parse_address(raw_words: &[u16; 3]) -> (AddresingMode, AddresingMode, u8) {
        let src_addressing_bits = raw_words[0].shr(4) & 0b11u16;
        let src_register_bits = raw_words[0].shr(7) & 0b1111u16;
        let src_register = Register::try_from(src_register_bits).unwrap();

        let (src_mode, first_extra_word_used) =
            parse_address(src_register, src_addressing_bits, raw_words).unwrap();

        let mut extra_words_used = if first_extra_word_used { 1u8 } else { 0u8 };

        let dst_addressing_bit = raw_words[0] & 0b10000000u16;
        let dst_register_bits = raw_words[0] & 0b1111u16;
        let dst_register = Register::try_from(dst_register_bits).unwrap();

        let dst_mode = if dst_addressing_bit == 0 {
            AddresingMode::Direct(dst_register)
        } else {
            let offset = raw_words[1 + usize::from(extra_words_used)];
            extra_words_used += 1;
            match dst_register {
                Register::Pc => AddresingMode::Symbolic(offset),
                Register::Sr => AddresingMode::Absolute(offset),
                _ => AddresingMode::Indexed((offset, dst_register)),
            }
        };

        (src_mode, dst_mode, extra_words_used)
    }

    fn emulated_form(&self) -> Option<EmulatedInstruction> {
        match self.operation {
            TwoOp::Mov => {
                if self.source == self.destination {
                    return Some(EmulatedInstruction {
                        operation: EmulatedOp::Nop,
                        data: None,
                        mode: None,
                    });
                }
                if self.source == AddresingMode::Indirect(Register::Sp) {
                    if self.destination == AddresingMode::Direct(Register::Pc) {
                        return Some(EmulatedInstruction {
                            operation: EmulatedOp::Ret,
                            data: None,
                            mode: None,
                        });
                    } else {
                        return Some(EmulatedInstruction {
                            operation: EmulatedOp::Pop,
                            data: Some(self.destination),
                            mode: Some(self.mode),
                        });
                    }
                } else if self.source == AddresingMode::Absolute(0) {
                    return Some(EmulatedInstruction {
                        operation: EmulatedOp::Clr,
                        data: Some(self.destination),
                        mode: Some(self.mode),
                    });
                } else if self.destination == AddresingMode::Direct(Register::Pc) {
                    return Some(EmulatedInstruction {
                        operation: EmulatedOp::Br,
                        data: Some(self.source),
                        mode: None,
                    });
                }
            }
            TwoOp::Bic => {
                if self.destination == AddresingMode::Direct(Register::Sr) {
                    if let AddresingMode::Immediate(bit) = self.source {
                        if bit == 1 {
                            return Some(EmulatedInstruction {
                                operation: EmulatedOp::Clrc,
                                data: None,
                                mode: None,
                            });
                        } else if bit == 2 {
                            return Some(EmulatedInstruction {
                                operation: EmulatedOp::Clrz,
                                data: None,
                                mode: None,
                            });
                        } else if bit == 4 {
                            return Some(EmulatedInstruction {
                                operation: EmulatedOp::Clrn,
                                data: None,
                                mode: None,
                            });
                        } else if bit == 8 {
                            return Some(EmulatedInstruction {
                                operation: EmulatedOp::Dint,
                                data: None,
                                mode: None,
                            });
                        }
                    }
                }
            }
            TwoOp::Bis => {
                if self.destination == AddresingMode::Direct(Register::Sr) {
                    if let AddresingMode::Immediate(bit) = self.source {
                        if bit == 1 {
                            return Some(EmulatedInstruction {
                                operation: EmulatedOp::Setc,
                                data: None,
                                mode: None,
                            });
                        } else if bit == 2 {
                            return Some(EmulatedInstruction {
                                operation: EmulatedOp::Setz,
                                data: None,
                                mode: None,
                            });
                        } else if bit == 4 {
                            return Some(EmulatedInstruction {
                                operation: EmulatedOp::Setn,
                                data: None,
                                mode: None,
                            });
                        } else if bit == 8 {
                            return Some(EmulatedInstruction {
                                operation: EmulatedOp::Eint,
                                data: None,
                                mode: None,
                            });
                        }
                    }
                }
            }
            TwoOp::Add => {
                if self.source == self.destination {
                    return Some(EmulatedInstruction {
                        operation: EmulatedOp::Rla,
                        data: Some(self.destination),
                        mode: Some(self.mode),
                    });
                }
                if let AddresingMode::Absolute(amount) = self.source {
                    if amount == 1 {
                        return Some(EmulatedInstruction {
                            operation: EmulatedOp::Inc,
                            data: Some(self.destination),
                            mode: Some(self.mode),
                        });
                    } else {
                        return Some(EmulatedInstruction {
                            operation: EmulatedOp::Incd,
                            data: Some(self.destination),
                            mode: Some(self.mode),
                        });
                    }
                }
            }
            TwoOp::Addc => {
                if self.source == self.destination {
                    return Some(EmulatedInstruction {
                        operation: EmulatedOp::Rlc,
                        data: Some(self.destination),
                        mode: Some(self.mode),
                    });
                }
                if self.source == AddresingMode::Absolute(0) {
                    return Some(EmulatedInstruction {
                        operation: EmulatedOp::Adc,
                        data: Some(self.destination),
                        mode: Some(self.mode),
                    });
                }
            }
            TwoOp::Sub => {
                if let AddresingMode::Absolute(amount) = self.source {
                    if amount == 1 {
                        return Some(EmulatedInstruction {
                            operation: EmulatedOp::Dec,
                            data: Some(self.destination),
                            mode: Some(self.mode),
                        });
                    } else {
                        return Some(EmulatedInstruction {
                            operation: EmulatedOp::Decd,
                            data: Some(self.destination),
                            mode: Some(self.mode),
                        });
                    }
                }
            }
            TwoOp::Subc => {
                if self.source == AddresingMode::Absolute(0) {
                    return Some(EmulatedInstruction {
                        operation: EmulatedOp::Sbc,
                        data: Some(self.destination),
                        mode: Some(self.mode),
                    });
                }
            }
            TwoOp::Dadd => {
                if self.source == AddresingMode::Absolute(0) {
                    return Some(EmulatedInstruction {
                        operation: EmulatedOp::Dadc,
                        data: Some(self.destination),
                        mode: Some(self.mode),
                    });
                }
            }
            TwoOp::Xor => {
                if self.source == AddresingMode::Absolute(0xffff) {
                    return Some(EmulatedInstruction {
                        operation: EmulatedOp::Inv,
                        data: Some(self.destination),
                        mode: Some(self.mode),
                    });
                }
            }
            TwoOp::Cmp => {
                if self.source == AddresingMode::Absolute(0) {
                    return Some(EmulatedInstruction {
                        operation: EmulatedOp::Tst,
                        data: Some(self.destination),
                        mode: Some(self.mode),
                    });
                }
            }
            _ => return None,
        }
        None
    }
}

#[derive(Debug)]
pub struct EmulatedInstruction {
    operation: EmulatedOp,
    data: Option<AddresingMode>,
    mode: Option<DataMode>,
}

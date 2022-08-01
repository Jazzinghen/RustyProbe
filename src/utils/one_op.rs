use crate::utils::data_address::{
    parse_address, AddresingMode, AsmInstruction, DataMode, Register,
};
use std::fmt;
use std::ops::{Shl, Shr};

// Single Operand instructions (Format II)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OneOp {
    Rrc,
    Swpb,
    Rra,
    Sxt,
    Push,
    Call,
    Reti,
}

impl TryFrom<u16> for OneOp {
    type Error = &'static str;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let op_data: u16 = value.shr(10);
        if op_data != 0b000100 {
            return Err("the provided word is not a one operand op code");
        }
        let masked_data: u16 = value.shr(7) & 0b111u16;
        match masked_data {
            0b000 => Ok(Self::Rrc),
            0b001 => Ok(Self::Swpb),
            0b010 => Ok(Self::Rra),
            0b011 => Ok(Self::Sxt),
            0b100 => Ok(Self::Push),
            0b101 => Ok(Self::Call),
            0b110 => Ok(Self::Reti),
            _ => Err("inexisting operation"),
        }
    }
}

impl From<OneOp> for u16 {
    fn from(val: OneOp) -> Self {
        let op_bits = 0b000100000u16
            + match val {
                OneOp::Rrc => 0b000u16,
                OneOp::Swpb => 0b001u16,
                OneOp::Rra => 0b010u16,
                OneOp::Sxt => 0b011u16,
                OneOp::Push => 0b100u16,
                OneOp::Call => 0b101u16,
                OneOp::Reti => 0b110u16,
            };
        op_bits.shl(7)
    }
}

impl From<OneOp> for String {
    fn from(val: OneOp) -> Self {
        match val {
            OneOp::Rrc => "rrc",
            OneOp::Swpb => "swpb",
            OneOp::Rra => "rra",
            OneOp::Sxt => "sxt",
            OneOp::Push => "push",
            OneOp::Call => "call",
            OneOp::Reti => "ret",
        }
        .to_string()
    }
}

impl fmt::Display for OneOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

#[derive(Debug)]
pub struct OneOpInstruction {
    operation: OneOp,
    data: AddresingMode,
    mode: Option<DataMode>,
}

impl AsmInstruction for OneOpInstruction {}

impl OneOpInstruction {
    pub fn new(raw_words: &[u16; 3]) -> (Self, bool) {
        let operation = OneOp::try_from(raw_words[0]).unwrap();
        let mode = if operation == OneOp::Rrc || operation == OneOp::Rra || operation == OneOp::Push
        {
            Some(DataMode::from(raw_words[0]))
        } else {
            None
        };

        let data_register = Register::try_from(raw_words[0] & 0b1111u16).unwrap();
        let addr_mode_bits = raw_words[0].shr(4) & 0b11u16;
        let (data, word_used) = parse_address(data_register, addr_mode_bits, raw_words).unwrap();

        (
            Self {
                operation,
                data,
                mode,
            },
            word_used,
        )
    }
}

use crate::utils::data_address::{get_signed_hex, AsmInstruction};
use std::fmt;
use std::ops::{Shl, Shr};

// Jumps
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JumpOp {
    Jne,
    Jeq,
    Jlo,
    Jhs,
    Jn,
    Jge,
    Jl,
    Jmp,
}

impl TryFrom<u16> for JumpOp {
    type Error = &'static str;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let op_data: u16 = value.shr(13);
        if op_data != 0b001 {
            return Err("the provided word is not a jump op code");
        }
        let masked_data: u16 = value.shr(10) & 0b111u16;
        match masked_data {
            0b000 => Ok(Self::Jne),
            0b001 => Ok(Self::Jeq),
            0b010 => Ok(Self::Jlo),
            0b011 => Ok(Self::Jhs),
            0b100 => Ok(Self::Jn),
            0b101 => Ok(Self::Jge),
            0b110 => Ok(Self::Jl),
            0b111 => Ok(Self::Jmp),
            _ => Err("somehow we managed to generate a value that's not a 3 bit u16"),
        }
    }
}

impl From<JumpOp> for u16 {
    fn from(val: JumpOp) -> Self {
        let op_bits = 0b001000u16
            + match val {
                JumpOp::Jne => 0b000u16,
                JumpOp::Jeq => 0b001u16,
                JumpOp::Jlo => 0b010u16,
                JumpOp::Jhs => 0b011u16,
                JumpOp::Jn => 0b100u16,
                JumpOp::Jge => 0b101u16,
                JumpOp::Jl => 0b110u16,
                JumpOp::Jmp => 0b111u16,
            };
        op_bits.shl(10)
    }
}

impl From<JumpOp> for String {
    fn from(val: JumpOp) -> Self {
        match val {
            JumpOp::Jne => "jne",
            JumpOp::Jeq => "jeq",
            JumpOp::Jlo => "jlo",
            JumpOp::Jhs => "jhs",
            JumpOp::Jn => "jn",
            JumpOp::Jge => "jge",
            JumpOp::Jl => "jl",
            JumpOp::Jmp => "jmp",
        }
        .to_string()
    }
}

impl fmt::Display for JumpOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

#[derive(Debug)]
pub struct JumpInstruction {
    operation: JumpOp,
    offset: u16,
}

impl JumpInstruction {
    fn new(word: u16) -> Self {
        let operation = JumpOp::try_from(word).unwrap();
        let offset_bits = word & 0b1111111111u16;
        let offset = if offset_bits & 0b1000000000u16 != 0 {
            offset_bits + 0b111111u16.shl(10)
        } else {
            offset_bits
        };

        Self { operation, offset }
    }
}

impl AsmInstruction for JumpInstruction {}

impl fmt::Display for JumpInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.operation, get_signed_hex(self.offset))
    }
}

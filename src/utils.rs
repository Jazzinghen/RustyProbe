use std::convert::{From, TryFrom};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    Pc,
    Sp,
    Sr,
    Cg,
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

impl TryFrom<u16> for Register {
    type Error = &'static str;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let op_data: u16 = value.shr(12);
        if op_data == 0b0001 {
            return Err("the provided word is not a two operand op code");
        }
        match op_data {
            0b0000 => Ok(Self::Pc),
            0b0001 => Ok(Self::Sp),
            0b0010 => Ok(Self::Sr),
            0b0011 => Ok(Self::Cg),
            0b0100 => Ok(Self::R4),
            0b0101 => Ok(Self::R5),
            0b0110 => Ok(Self::R6),
            0b0111 => Ok(Self::R7),
            0b1000 => Ok(Self::R8),
            0b1001 => Ok(Self::R9),
            0b1010 => Ok(Self::R10),
            0b1011 => Ok(Self::R11),
            0b1100 => Ok(Self::R12),
            0b1101 => Ok(Self::R13),
            0b1110 => Ok(Self::R14),
            0b1111 => Ok(Self::R15),
            _ => Err("inexisting operation"),
        }
    }
}

impl From<Register> for u16 {
    fn from(val: Register) -> Self {
        let op_bits = match val {
            Register::Pc => 0b0000u16,
            Register::Sp => 0b0001u16,
            Register::Sr => 0b0010u16,
            Register::Cg => 0b0011u16,
            Register::R4 => 0b0100u16,
            Register::R5 => 0b0101u16,
            Register::R6 => 0b0110u16,
            Register::R7 => 0b0111u16,
            Register::R8 => 0b1000u16,
            Register::R9 => 0b1001u16,
            Register::R10 => 0b1010u16,
            Register::R11 => 0b1011u16,
            Register::R12 => 0b1100u16,
            Register::R13 => 0b1101u16,
            Register::R14 => 0b1110u16,
            Register::R15 => 0b1111u16,
        };
        op_bits.shl(12)
    }
}

impl From<Register> for String {
    fn from(val: Register) -> Self {
        match val {
            Register::Pc => "PC",
            Register::Sp => "SP",
            Register::Sr => "SR",
            Register::Cg => "CG",
            Register::R4 => "R4",
            Register::R5 => "R5",
            Register::R6 => "R6",
            Register::R7 => "R7",
            Register::R8 => "R8",
            Register::R9 => "R9",
            Register::R10 => "R10",
            Register::R11 => "R11",
            Register::R12 => "R12",
            Register::R13 => "R13",
            Register::R14 => "R14",
            Register::R15 => "R15",
        }
        .to_string()
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddresingMode {
    Direct(Register),
    Indexed((i16, Register)),
    Indirect(Register),
    Autoincrement(Register),
    Absolute(u16),
    Symbolic(i16),
    Immediate(i16),
}

impl fmt::Display for AddresingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Direct(reg) => write!(f, "{}", String::from(reg)),
            Self::Indexed((offset, reg)) => write!(f, "{:+#x}({})", offset, String::from(reg)),
            Self::Indirect(reg) => write!(f, "@{}", String::from(reg)),
            Self::Autoincrement(reg) => write!(f, "@{}+", String::from(reg)),
            Self::Absolute(address) => write!(f, "&{:#x}", address),
            Self::Symbolic(offset) => write!(f, "{:+#x}", offset),
            Self::Immediate(value) => write!(f, "#{}", value),
        }
    }
}

#[derive(Debug)]
pub enum DataMode {
    Byte,
    Word,
}

impl fmt::Display for DataMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Byte => write!(f, ".b"),
            Self::Word => write!(f, ".w"),
        }
    }
}

#[derive(Debug)]
pub struct JumpInstruction {
    operation: JumpOp,
    offset: i16,
}

impl fmt::Display for JumpInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.operation, self.offset)
    }
}

#[derive(Debug)]
pub struct OneOpInstruction {
    operation: OneOp,
    data: Option<AddresingMode>,
    mode: Option<AddresingMode>,
}

#[derive(Debug)]
pub struct TwoOpInstruction {
    operation: TwoOp,
    source: Option<AddresingMode>,
    destination: Option<AddresingMode>,
    mode: Option<AddresingMode>,
}

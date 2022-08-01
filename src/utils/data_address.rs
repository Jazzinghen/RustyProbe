use std::fmt;

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
        if value >= 0b10000 {
            return Err("the provided value is not a register identifier");
        }
        match value {
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
            _ => Err("inexisting register"),
        }
    }
}

impl From<Register> for u16 {
    fn from(val: Register) -> Self {
        match val {
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
        }
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
    Indexed((u16, Register)),
    Indirect(Register),
    Autoincrement(Register),
    Absolute(u16),
    Symbolic(u16),
    Immediate(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataMode {
    Byte,
    Word,
}

impl From<u16> for DataMode {
    fn from(value: u16) -> Self {
        let size_data = value & 0b1000000u16;
        if size_data == 0 {
            Self::Word
        } else {
            Self::Byte
        }
    }
}

impl From<DataMode> for u16 {
    fn from(val: DataMode) -> Self {
        if val == DataMode::Byte {
            0b1000000u16
        } else {
            0u16
        }
    }
}

impl From<DataMode> for String {
    fn from(val: DataMode) -> Self {
        if val == DataMode::Word {
            String::from(".w")
        } else {
            String::from(".b")
        }
    }
}

impl fmt::Display for DataMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(*self))
    }
}

pub trait AsmInstruction {}

pub fn parse_address(
    register: Register,
    addr_bits: u16,
    raw_words: &[u16; 3],
) -> Result<(AddresingMode, bool), &'static str> {
    let mut extra_word_used = false;

    let mode = match addr_bits {
        0b00 => {
            if register == Register::Cg {
                AddresingMode::Immediate(0)
            } else {
                AddresingMode::Direct(register)
            }
        }
        0b01 => match register {
            Register::Pc => {
                extra_word_used = true;
                AddresingMode::Symbolic(raw_words[1])
            }
            Register::Sr => {
                extra_word_used = true;
                AddresingMode::Absolute(raw_words[1])
            }
            Register::Cg => AddresingMode::Immediate(1),
            _ => {
                extra_word_used = true;
                AddresingMode::Indexed((raw_words[1], register))
            }
        },
        0b10 => match register {
            Register::Sr => AddresingMode::Immediate(4),
            Register::Cg => AddresingMode::Immediate(2),
            _ => AddresingMode::Indirect(register),
        },
        0b11 => match register {
            Register::Pc => {
                extra_word_used = true;
                AddresingMode::Immediate(raw_words[1])
            }
            Register::Sr => AddresingMode::Immediate(8),
            Register::Cg => AddresingMode::Immediate(-1i16 as u16),
            _ => AddresingMode::Autoincrement(register),
        },
        _ => return Err("provided an invalid addressing mode value"),
    };

    Ok((mode, extra_word_used))
}

pub fn get_signed_hex(src: u16) -> String {
    let signed = src as i16;
    let abs = signed.abs() as u16;
    let sign = if signed < 0 { "-" } else { "" };
    format!("{}{:+#x}", sign, abs)
}

impl fmt::Display for AddresingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Direct(reg) => write!(f, "{}", String::from(reg)),
            Self::Indexed((offset, reg)) => {
                write!(f, "{}({})", get_signed_hex(offset), String::from(reg))
            }
            Self::Indirect(reg) => write!(f, "@{}", String::from(reg)),
            Self::Autoincrement(reg) => write!(f, "@{}+", String::from(reg)),
            Self::Absolute(address) => write!(f, "&{:#x}", address),
            Self::Symbolic(offset) => write!(f, "{}", get_signed_hex(offset)),
            Self::Immediate(value) => write!(f, "#{}", value as i16),
        }
    }
}

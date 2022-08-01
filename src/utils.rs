use std::convert::{From, TryFrom};
use std::fmt;
use std::ops::{Add, Shl, Shr};

pub mod two_op;
use two_op::TwoOpInstruction;

pub mod one_op;
use one_op::OneOpInstruction;

pub mod jumps;
use jumps::JumpInstruction;

pub mod data_address;

#[derive(Debug)]
enum Instruction {
    Jump(JumpInstruction),
    OneOp(OneOpInstruction),
    TwoOp(TwoOpInstruction),
}

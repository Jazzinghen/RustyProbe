use itertools::Itertools;
use std::io::{self, BufRead};
use std::ops::{Shl, Shr};

use crate::utils::{jumps, one_op, two_op};

pub fn disassemble<T: BufRead>(
    reader: &mut T,
    pc_base: u16,
    quiet: bool,
) -> io::Result<Vec<String>> {
    let mut raw_data: Vec<u8> = Vec::new();
    reader.read_to_end(&mut raw_data)?;

    let word_data: Vec<u16> = raw_data
        .into_iter()
        .tuples()
        .map(|(low, high)| u16::from(high).shl(8) + u16::from(low))
        .collect_vec();

    let mut first_pass_decoded: Vec<String> = Vec::new();
    let mut curr_word: usize = 0;
    while curr_word < word_data.len() {
        let (step, decoded_op) =
            disassemble_op(&word_data[curr_word..(curr_word + 3).min(word_data.len())]);
        for s in 0..3 {
            if s < step {
                print!("{:#06x} ", word_data[curr_word + s]);
            } else {
                print!("       ");
            }
        }
        println!("{}", decoded_op);
        first_pass_decoded.push(decoded_op);

        curr_word += step;
    }
    Ok(first_pass_decoded)
}

fn disassemble_op(raw_words: &[u16]) -> (usize, String) {
    let jmp_bits: u16 = raw_words[0].shr(13);
    if jmp_bits == 0b001 {
        let jump_instruction = jumps::JumpInstruction::new(raw_words[0]);
        return (1, format!("{}", jump_instruction));
    };
    let one_op_bits: u16 = raw_words[0].shr(10);
    if one_op_bits == 0b000100 {
        let (one_op_instruction, extra_word) = one_op::OneOpInstruction::new(raw_words);
        return (
            if extra_word { 2 } else { 1 },
            format!("{}", one_op_instruction),
        );
    }
    let (two_op_instruction, extra_words) = two_op::TwoOpInstruction::new(raw_words);
    let op_string = if let Some(emulated) = two_op_instruction.emulated_form() {
        format!("{}", emulated)
    } else {
        format!("{}", two_op_instruction)
    };
    (1 + usize::from(extra_words), op_string)
}

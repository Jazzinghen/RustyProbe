use itertools::Itertools;
use std::{
    io::{self, Read},
    ops::Shl,
};

pub fn disassemble(
    reader: &mut Box<dyn Read + 'static>,
    pc_base: u16,
    quiet: bool,
) -> io::Result<Vec<String>> {
    let mut raw_data: Vec<u8> = Vec::new();
    reader.as_mut().read_to_end(&mut raw_data)?;

    let word_data: Vec<u16> = raw_data
        .into_iter()
        .tuples()
        .map(|(low, high)| u16::from(high).shl(8) + u16::from(low))
        .collect_vec();

    let mut first_pass_decoded: Vec<String> = Vec::new();
    let mut curr_word: usize = 0;
    while curr_word < word_data.len() {
        let (step, decoded_op) = disassemble_op(&word_data[curr_word..curr_word + 3]);
        first_pass_decoded.push(decoded_op);

        curr_word += step;
    }
    Ok(vec![])
}

fn disassemble_op(raw_bytes: &[u16]) -> (usize, String) {
    (0, String::new())
}

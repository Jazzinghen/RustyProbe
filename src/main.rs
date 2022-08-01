use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::num::ParseIntError;
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

mod disassembler;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    mode: Mode,

    /// Path to the file where to write the result, if missing it will be output to stdin
    #[clap(short, long, value_parser, value_name = "OUTPUT")]
    output: Option<PathBuf>,

    /// Base instruction pointer for (dis)assembly.
    #[clap(
        short,
        long,
        default_value_t = 0u16,
        parse(try_from_str=from_dec_or_hex),
        value_name = "CP_BASE"
    )]
    base_pointer: u16,

    /// Do not print logs or output
    #[clap(short, long, action)]
    quiet: bool,
}

#[derive(Subcommand, Debug)]
enum Mode {
    /// Converts an assembly file to hex
    Assemble(AssembleConfig),
    /// Disassembles a binary file to human-readable assembly file
    Disassemble(DisassembleConfig),
}

#[derive(Debug, Args)]
struct AssembleConfig {
    #[clap(parse(try_from_str=check_and_canonicalize), value_name = "SOURCE")]
    file_path: Option<PathBuf>,
}

#[derive(Debug, Args)]
struct DisassembleConfig {
    #[clap(parse(try_from_str=check_and_canonicalize), value_name = "SOURCE")]
    file_path: Option<PathBuf>,

    #[clap(long, parse(try_from_str=from_dec_or_hex), requires = "stack", value_name = "SP_BASE")]
    stack_begin: Option<u16>,

    #[clap(long, parse(try_from_str=from_dec_or_hex), group = "stack", value_name = "SP_END")]
    stack_end: Option<u16>,

    #[clap(long, parse(try_from_str=from_dec_or_hex), group = "stack", value_name = "SP_SIZE")]
    stack_size: Option<u16>,

    #[clap(long, value_parser, action)]
    ignore_ivt: bool,

    #[clap(long, value_parser, action)]
    ignore_special_functions: bool,

    #[clap(long, value_parser, action)]
    ignore_peripherals: bool,
}

fn check_and_canonicalize(s: &str) -> std::io::Result<PathBuf> {
    let actual_path = PathBuf::from(s);
    actual_path.canonicalize()
}

fn from_dec_or_hex(s: &str) -> Result<u16, ParseIntError> {
    if let Ok(decimal) = s.parse::<u16>() {
        Ok(decimal)
    } else {
        u16::from_str_radix(s.to_uppercase().trim_start_matches("0X"), 16)
    }
}

fn main() {
    let user_configs = Cli::parse();

    if let Mode::Disassemble(config) = user_configs.mode {
        if let Some(input) = config.file_path {
            let f = File::open(input).unwrap();
            let mut reader = BufReader::new(f);

            let disassembled = disassembler::disassemble(
                &mut reader,
                user_configs.base_pointer,
                user_configs.quiet,
            )
            .unwrap();
        };
    }

    println!("Done");
}

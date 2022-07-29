use clap::{Parser, Subcommand};
use std::path::PathBuf;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    mode: Mode,

    #[clap(short, long, value_parser, value_name = "OUTPUT")]
    output: Option<PathBuf>,

    #[clap(
        short,
        long,
        default_value_t = 0u16,
        value_parser,
        value_name = "BASE_CP"
    )]
    base_pointer: u16,

    #[clap(short, long, action)]
    silent: bool,
}

#[derive(Subcommand, Debug)]
enum Mode {
    Assemble {
        #[clap(parse(try_from_str=check_and_canonicalize), value_name = "SOURCE")]
        file_path: Option<PathBuf>,
    },
    Disassemble {
        #[clap(parse(try_from_str=check_and_canonicalize), value_name = "SOURCE")]
        file_path: Option<PathBuf>,
    },
}

fn check_and_canonicalize(s: &str) -> std::io::Result<PathBuf> {
    let actual_path = PathBuf::from(s);
    actual_path.canonicalize()
}

fn main() {
    let user_configs = Cli::parse();

    println!("Config: {:?}", user_configs);
}

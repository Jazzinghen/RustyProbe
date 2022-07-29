use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
mod utils;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    mode: Mode,
}

#[derive(Subcommand, Debug)]
enum Mode {
    /// Converts an assembly file to hex
    Assemble(ModeConfig),
    /// Disassembles a binary file to human-readable assembly file
    Disassemble(ModeConfig),
}

#[derive(Debug, Args)]
#[clap(args_conflicts_with_subcommands = true)]
struct ModeConfig {
    /// Path to the file where to write the result, if missing it will be output to stdin
    #[clap(short, long, value_parser, value_name = "OUTPUT")]
    output: Option<PathBuf>,

    /// Base instruction pointer for (dis)assembly.
    #[clap(
        short,
        long,
        default_value_t = 0u16,
        value_parser,
        value_name = "BASE_CP"
    )]
    base_pointer: u16,

    /// Do not print logs or output
    #[clap(short, long, action)]
    quiet: bool,

    #[clap(parse(try_from_str=check_and_canonicalize), value_name = "SOURCE")]
    file_path: Option<PathBuf>,
}

fn check_and_canonicalize(s: &str) -> std::io::Result<PathBuf> {
    let actual_path = PathBuf::from(s);
    actual_path.canonicalize()
}

fn main() {
    let user_configs = Cli::parse();

    println!("Config: {:?}", user_configs);
}

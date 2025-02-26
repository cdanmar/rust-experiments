// use clap::{ArgAction, Parser};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `head`
struct Args {
    /// Input files
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(
        long,
        short('n'),
        value_parser = clap::value_parser!(u64).range(1..),
        default_value_t = 10,
        conflicts_with = "bytes"
    )]
    lines: u64,
    
    /// <BYTES> Number of bytes
    #[arg(
        long,
        short('c'),
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>,
}



fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}

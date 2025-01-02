use clap::{ArgAction, Parser};
// use clap_stdin::MaybeStdin;
use anyhow::Result;
// use predicates::path::FileContentPredicate;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `cat`
struct Args {
    /// Input files
    #[arg(value_name = "FILE", default_value = "-")]
    // files: Vec<MaybeStdin<String>>,
    files: Vec<String>,

    /// Number the output lines, starting at 1
    #[arg(
        long,
        short('n'),
        action = ArgAction::SetTrue,
        conflicts_with = "number_nonblank_lines"
    )]
    number_lines: bool,

    /// Number the non-blank output lines, starting at 1
    #[arg(long, short('b'), action = ArgAction::SetTrue, conflicts_with = "number_lines")]
    number_nonblank_lines: bool,
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(_) => {
                let file = File::open(filename)?;
                let reader = BufReader::new(file);
                let mut count = 0;
                for line in reader.lines() {
                    match line {
                        Err(e) => eprintln!("Error reading line: {}", e),
                        // Ok(content) => println!("{}", content),
                        Ok(content) => {
                            if args.number_lines {
                                count += 1;
                                println!("{:>6}\t{}", count, content);
                            } else if args.number_nonblank_lines && is_not_blank(&content) {
                                count += 1;
                                println!("{:>6}\t{}", count, content);
                            } else {
                                println!("{}", content);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn is_not_blank(s: &str) -> bool {
    !s.trim().is_empty()
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
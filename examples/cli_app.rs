use clap::Parser;
use std::io::prelude::*;
use std::{io::BufReader, path::PathBuf};

/// Search for a pattern in a file and display lines that contain it.
#[derive(Parser)]
struct Cli {
    // todo: sub-commands
    /// The pattern to look for.
    pattern: String,
    /// The path to the file to read.
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let file = std::fs::File::open(&args.path).expect("Could not open file"); // todo: return an exitcode here instead of panicking
    let reader = BufReader::new(file);

    let mut at_least_one_result = false;
    for (line_number, line) in reader.lines().enumerate() {
        let line = line.expect("Could not read line"); // todo: same as above
        if line.contains(&args.pattern) {
            println!("[line {}]: {}", line_number, line.trim());
            at_least_one_result = true;
        }
    }

    let status = match at_least_one_result {
        true => exitcode::OK,
        false => 1,
    };
    std::process::exit(status);
}

use clap::{Parser, Subcommand};
use std::io::prelude::*;
use std::{io::BufReader, path::PathBuf};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for a pattern in a file and display lines that contain it.
    Grep {
        /// The pattern to look for
        pattern: String,
        /// The path to the file
        path: PathBuf,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let status = match &cli.command {
        Commands::Grep { pattern, path } => grep(pattern, path),
    };
    std::process::exit(status);
}

fn grep(pattern: &String, path: &PathBuf) -> exitcode::ExitCode {
    let file = std::fs::File::open(path).expect("Could not open file"); // todo: return an exitcode here instead of panicking
    let reader = BufReader::new(file);

    let mut at_least_one_result = false;
    for (line_number, line) in reader.lines().enumerate() {
        let line = line.expect("Could not read line"); // todo: same as above
        if line.contains(pattern) {
            println!("[line {}]: {}", line_number + 1, line.trim());
            at_least_one_result = true;
        }
    }

    match at_least_one_result {
        true => exitcode::OK,
        false => 1,
    }
}

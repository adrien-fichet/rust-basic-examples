use clap::{Parser, Subcommand};
use std::io::prelude::*;
use std::{io::BufReader, path::PathBuf};
use std::error::Error;

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
    /// Returns with an exist code of zero
    True,
    /// Returns with a non-zero exit code
    False,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let status = match &cli.command {
        Commands::Grep { pattern, path } => grep(pattern, path),
        Commands::True => exitcode::OK,
        Commands::False => 1,
    };
    std::process::exit(status);
}

fn grep(pattern: &String, path: &PathBuf) -> exitcode::ExitCode {
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Could not open file: {}", err);
            return exitcode::IOERR;
        }
    };
    let reader = BufReader::new(file);

    let mut at_least_one_result = false;
    for (line_number, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                eprintln!("Could not read line: {}", err);
                return exitcode::IOERR;
            }
        };
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

use clap::{Parser, Subcommand};
use std::error::Error;
use std::io::prelude::*;
use std::{io::BufReader, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// A CLI app example
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
    /// Return with an exist code of zero
    True,
    /// Return with a non-zero exit code
    False,
    /// Say hello!
    Hello,
    /// Write arguments to the standard output
    Echo {
        text: Vec<String>,
        #[clap(short = 'n')]
        /// Do not print the trailing newline character
        omit_newline: bool,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let status = match &cli.command {
        Commands::Grep { pattern, path } => grep(pattern, path),
        Commands::True => exitcode::OK,
        Commands::False => 1,
        Commands::Hello => hello(),
        Commands::Echo { text, omit_newline } => echo(text, omit_newline),
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

fn hello() -> exitcode::ExitCode {
    println!("Hello!");
    exitcode::OK
}

fn echo(text: &[String], omit_newline: &bool) -> exitcode::ExitCode {
    print!("{}{}", text.join(" "), if *omit_newline { "" } else { "\n" });
    exitcode::OK
}

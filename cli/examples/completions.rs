use clap::{CommandFactory, Parser};
use clap_complete::aot::Zsh;
use std::io;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'v', long)]
    verbose: bool,
}

fn main() {
    let cmd = &mut Cli::command();
    clap_complete::generate(Zsh, cmd, "completions".to_string(), &mut io::stdout())
}

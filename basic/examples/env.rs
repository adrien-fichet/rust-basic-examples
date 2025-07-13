// Usage: cargo run --example env -- arg1 arg2
// https://doc.rust-lang.org/stable/std/env/index.html

use dotenvy::dotenv;
use std::env;
use std::env::{VarError, consts};
use std::path::Path;

fn main() {
    dotenv().ok(); // read a .env file and inject its vars into the current env
    assert_ne!(env::var("HOME").unwrap(), "/"); // but do not override already set env variables
    assert_ne!(env::var("FOO").unwrap(), "buz"); // and take the first value found

    env_constants();
    env_macros();
    println!("---");
    read_update_delete_env_var();
    println!("---");

    env::set_current_dir(Path::new("/tmp")).expect("Could not chdir to /tmp");
    println!(
        "Current dir: {:?}",
        env::current_dir().expect("Current directory not found")
    );
    println!("Exe path: {:?}", env::current_exe().expect("Exe path not found"));
    println!("Args: {:?}", env::args().collect::<Vec<_>>());
    println!("Temp dir: {:?}", env::temp_dir());
    println!("---");

    for (key, value) in env::vars().filter(|(key, _)| key.starts_with("CARGO") || key.starts_with("RUST")) {
        println!("{key}={value}");
    }
}

fn env_constants() {
    println!("ARCH: {}", consts::ARCH);
    println!("FAMILY: {}", consts::FAMILY);
    println!("OS: {}", consts::OS);
}

fn env_macros() {
    println!("PWD at compile time is: {}", env!("PWD")); // will panic if the variable is not defined
    let foo_env = option_env!("FOO");
    match foo_env {
        Some(value) => println!("The value of FOO at compile time is: {value}"),
        None => println!("FOO was not defined at compile time"),
    }
}

fn read_update_delete_env_var() {
    let key = "FOO";
    let env_var = env::var(key);
    match env_var {
        Ok(value) => println!("{key} was found in the current env with the value \"{value}\""),
        Err(VarError::NotPresent) => eprintln!("{key} was not found in the current env"),
        Err(VarError::NotUnicode(_)) => {
            eprintln!("{key} was found in the current env but is not a valid unicode string")
        }
    }

    unsafe { env::set_var(key, "bar") };
    println!("{key} has now the value \"{}\"", env::var(key).unwrap());

    unsafe { env::remove_var(key) };
    match env::var(key) {
        Err(VarError::NotPresent) => println!("{key} was removed from the current env"),
        _ => eprintln!("{key} was not removed from the current env"),
    }
}

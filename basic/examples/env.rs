// Usage: cargo run --example env -- arg1 arg2
// https://doc.rust-lang.org/stable/std/env/index.html

use dotenvy::dotenv;
use std::env;
use std::env::{consts, VarError};
use std::path::Path;

fn main() {
    dotenv().ok(); // read a .env file and inject its vars into the current env
    assert_ne!(env::var("HOME").unwrap(), "/"); // but do not override already set env variables
    assert_ne!(env::var("FOO").unwrap(), "buz"); // and take the first value found

    env_constants();
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
        println!("{}={}", key, value);
    }
}

fn env_constants() {
    println!("ARCH: {}", consts::ARCH);
    println!("FAMILY: {}", consts::FAMILY);
    println!("OS: {}", consts::OS);
}

fn read_update_delete_env_var() {
    let key = "FOO";
    let env_var = env::var(key);
    match env_var {
        Ok(value) => println!("{} was found in the current env with the value \"{}\"", key, value),
        Err(VarError::NotPresent) => println!("{} was not found in the current env", key),
        Err(VarError::NotUnicode(_)) => {
            println!("{} was found in the current env but is not a valid unicode string", key)
        }
    }

    env::set_var(key, "bar");
    println!("{} has now the value \"{}\"", key, env::var(key).unwrap());

    env::remove_var(key);
    match env::var(key) {
        Err(VarError::NotPresent) => println!("{} was removed from the current env", key),
        _ => println!("{} was not removed from the current env", key),
    }
}

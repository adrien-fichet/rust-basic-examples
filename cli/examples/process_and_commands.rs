use anyhow::Result;
use std::process::Command;

fn main() -> Result<()> {
    wait_for_child()?;
    Ok(())
}

fn wait_for_child() -> Result<()> {
    println!("Start");
    Command::new("sleep").arg("0.5").spawn()?.wait()?;
    println!("End");
    Ok(())
}

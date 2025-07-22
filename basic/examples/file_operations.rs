// append to file
// create tempdir

use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Seek, SeekFrom, Write};
use std::process::Command;

fn main() -> Result<()> {
    write_and_read_tempfile()?;
    create_empty_file()?;
    time_since_last_modified()?;
    Ok(())
}

fn write_and_read_tempfile() -> Result<()> {
    let mut tmp_file = tempfile::tempfile().expect("Could not create temp file");
    println!("{tmp_file:?}");

    ["Hello, world!", "Line number 2"]
        .iter()
        .for_each(|line| writeln!(tmp_file, "{line}").expect("Could not write to temp file"));

    tmp_file.seek(SeekFrom::Start(0)).expect("Could not seek temp file");

    let buf_reader = BufReader::new(tmp_file);
    let mut lines_iter = buf_reader.lines();
    assert_eq!(lines_iter.next().unwrap()?, "Hello, world!".to_string());
    assert_eq!(lines_iter.next().unwrap()?, "Line number 2".to_string());
    Ok(())
}

fn create_empty_file() -> Result<()> {
    File::create("/tmp/empty_file").expect("Could not create empty file");
    assert!(Command::new("ls").arg("/tmp/empty_file").status()?.success());
    Ok(())
}

fn time_since_last_modified() -> Result<()> {
    let file = File::open("/etc/hosts")?;
    let metadata = file.metadata()?;
    let last_modified = metadata.modified()?;
    let duration_since_last_modified = last_modified.elapsed()? / 3600 / 24;
    println!("Last modified: {last_modified:?}");
    println!("Duration since last modified: {duration_since_last_modified:?} days");
    Ok(())
}

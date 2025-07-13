// append to file
// create tempdir

use std::io::prelude::*;
use std::io::{BufReader, Seek, SeekFrom, Write};

fn write_and_read_tempfile() -> Result<(), Box<dyn std::error::Error>> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    write_and_read_tempfile()?;
    Ok(())
}

// append to file
// create tempdir

use std::io::{Read, Seek, SeekFrom, Write, BufReader};

fn write_and_read_tempfile() {
    let mut tmp_file = tempfile::tempfile().expect("Could not create temp file");
    ["Hello, world!", "Line number 2"]
        .iter()
        .for_each(|line| writeln!(tmp_file, "{}", line).expect("Could not write to temp file"));

    tmp_file
        .seek(SeekFrom::Start(0))
        .expect("Could not seek temp file");

    let mut buf_reader = BufReader::new(tmp_file);
    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Could not read temp file");

    assert_eq!(contents, "Hello, world!\nLine number 2\n".to_string());
}

fn main() {
    write_and_read_tempfile();

    todo!();
}

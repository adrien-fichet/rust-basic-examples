use assert_cmd::prelude::*;
use escargot::CargoBuild;
use std::error::Error;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_true() {
    let cli_app = CargoBuild::new().example("cli_app").run().unwrap();
    cli_app.command().arg("true").assert().success();
}

#[test]
fn test_false() {
    let cli_app = CargoBuild::new().example("cli_app").run().unwrap();
    cli_app.command().arg("false").assert().failure();
}

#[test]
fn test_grep_pattern_found() -> Result<(), Box<dyn Error>> {
    let mut tmp_file = NamedTempFile::new()?;
    writeln!(tmp_file, "Hello, world!")?;
    let cli_app = CargoBuild::new().example("cli_app").run()?;
    cli_app
        .command()
        .arg("grep")
        .arg("Hello")
        .arg(tmp_file.path())
        .assert()
        .success()
        .stdout(predicates::str::contains("[line 1]"));
    Ok(())
}

#[test]
fn test_grep_pattern_not_found() -> Result<(), Box<dyn Error>> {
    let mut tmp_file = NamedTempFile::new()?;
    writeln!(tmp_file, "Hello, world!")?;
    let cli_app = CargoBuild::new().example("cli_app").run()?;
    cli_app
        .command()
        .arg("grep")
        .arg("hello")
        .arg(tmp_file.path())
        .assert()
        .failure()
        .code(1);
    Ok(())
}

#[test]
fn test_grep_file_not_found() -> Result<(), Box<dyn Error>> {
    let cli_app = CargoBuild::new().example("cli_app").run()?;
    cli_app
        .command()
        .arg("grep")
        .arg("hello")
        .arg("does_not_exist.txt")
        .assert()
        .failure()
        .code(exitcode::IOERR);
    Ok(())
}

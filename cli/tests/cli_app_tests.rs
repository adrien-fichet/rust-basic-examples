use anyhow::Result;
use assert_cmd::prelude::*;
use escargot::CargoBuild;
use std::io::Write;
use std::process::Command;
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
fn test_grep_pattern_found() -> Result<()> {
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
fn test_grep_pattern_not_found() -> Result<()> {
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
fn test_grep_file_not_found() -> Result<()> {
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

#[test]
fn test_hello() {
    let cli_app = CargoBuild::new().example("cli_app").run().unwrap();
    cli_app.command().arg("hello").assert().success().stdout("Hello!\n");
}

#[test]
fn test_echo_no_args() {
    let cli_app = CargoBuild::new().example("cli_app").run().unwrap();
    cli_app.command().arg("echo").assert().success().stdout("\n");
}

#[test]
fn test_echo_with_args() -> Result<()> {
    let cli_app = CargoBuild::new().example("cli_app").run()?;
    let args_values = [
        ["Hello", "there"],
        ["Hello     ", " there"],
        ["Hello there", ""],
        ["Hello -n  ", "there"],
    ];

    for args in args_values {
        let expected = Command::new("echo").args(args).output()?.stdout;
        cli_app
            .command()
            .arg("echo")
            .args(args)
            .assert()
            .success()
            .stdout(expected);
    }
    Ok(())
}

#[test]
fn test_echo_impl_differences() -> Result<()> {
    let cli_app = CargoBuild::new().example("cli_app").run()?;
    let args = ["Hello there", "-n"];
    let expected = Command::new("echo").args(args).output()?.stdout;
    // the -n option only works when specified at the start in the original echo command
    assert_ne!(cli_app.command().arg("echo").args(args).output()?.stdout, expected);
    Ok(())
}

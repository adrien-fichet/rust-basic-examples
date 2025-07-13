use anyhow::{Context, Result};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;

fn main() -> Result<()> {
    wait_for_child()?;
    script_with_source()?;
    Ok(())
}

fn wait_for_child() -> Result<()> {
    println!("Start");
    Command::new("sleep").arg("0.5").spawn()?.wait()?;
    println!("End");
    Ok(())
}

fn script_with_source() -> Result<()> {
    let mut source_script = NamedTempFile::new().with_context(|| "Could not create temp file")?;
    writeln!(
        source_script,
        indoc::indoc! {"
            export MY_VAR=42
            export MY_NAME=Riki
        "}
    )
    .with_context(|| "Could not write to temp file")?;

    let source_script_path = source_script.path().to_str().unwrap();
    let script_code = indoc::formatdoc! {r#"
        source {source_script_path}
        pwd | tr -d '\n'
        echo -n $MY_VAR $MY_NAME >&2
    "#};
    let args = vec![];
    let options = run_script::ScriptOptions {
        runner: Some("/bin/bash".to_string()),
        working_directory: Some(Path::new("/tmp").to_path_buf()),
        print_commands: true,
        exit_on_error: true,
        ..run_script::ScriptOptions::new()
    };
    let (code, output, error) = run_script::run(&script_code, &args, &options)?;
    println!("exit code: {code}");
    println!("stdout: {output}");
    println!("stderr: {error}");
    Ok(())
}

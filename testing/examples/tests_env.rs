use anyhow::Result;
use std::{env, process::Command};

fn main() -> Result<()> {
    launch_tests(false)?;
    launch_tests(true)?;
    Ok(())
}

fn launch_tests(nextest: bool) -> Result<()> {
    unsafe {
        env::set_var("FOO", "bar");
    }

    let exe = env::current_exe()?;
    let workspace_dir = exe.join("../../../../").canonicalize()?;

    let mut test_subcommand = if nextest { vec!["nextest", "run"] } else { vec!["test"] };
    test_subcommand.extend(vec!["--example", "tests_env"]);

    Command::new("cargo")
        .args(test_subcommand)
        .current_dir(workspace_dir)
        .spawn()?
        .wait()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn test_001() {
        assert!(env::var("FOO").unwrap() == "bar");
        unsafe {
            env::set_var("BAR", "baz");
        }
    }

    #[test]
    fn test_002() {
        assert!(env::var("FOO").unwrap() == "bar");
        let res = env::var("BAR");
        assert!(res.is_err());
    }
}

// cargo nextest run --example nextest
// Nextest config file: .config/nextest.toml (from workspace root)

fn main() {
    todo!();
}

#[test]
fn test_001() {
    println!("stdout from test_001");
    eprintln!("stderr from test_001");
    assert!(true);
}

#[test]
fn test_002() {
    println!("stdout from test_002");
    eprintln!("stderr from test_002");
    assert!(false);
}

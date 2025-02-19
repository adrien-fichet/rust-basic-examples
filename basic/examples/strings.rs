fn main() {
    let s = "The quick brown fox jumps over the lazy dog".to_string();
    assert_eq!(s.chars().nth(2), Some('e')); // get the character at a specific index
    assert_eq!(s.get(4..=8), Some("quick")); // extract a slice

    todo!();
}

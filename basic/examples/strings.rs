// https://doc.rust-lang.org/stable/std/primitive.str.html

fn main() {
    let s = "The quick brown fox jumps over the lazy dog".to_string();
    extract(&s);
    lengths(&s);
    search_and_replace(&s);
    convert();
    chars(&s);
    split(&s);
    split_every_n_chars(&s, 3);
    trim_strip();
    transforms();
    multiline_strings();
}

fn split(s: &str) {
    let (head, tail) = s.split_at(10); // 10 bytes offset from start
    assert_eq!(s.chars().nth(10), Some('b'));
    assert_eq!(head, "The quick ");
    assert_eq!(tail, "brown fox jumps over the lazy dog");

    assert_eq!(s.split("fox").next(), Some("The quick brown "));
    assert_eq!("mount=type=bind".rsplit_once("="), Some(("mount=type", "bind")));

    assert_eq!(s.split_whitespace().next(), Some("The"));
}

fn split_every_n_chars(s: &str, n: usize) {
    let mut res = String::new();
    for (i, c) in s.chars().enumerate() {
        res.push(c);
        if (i + 1) % n == 0 {
            res.push('\n');
        }
    }
    assert_eq!(res[..=18], ["The", " qu", "ick", " br", "own"].join("\n"));
}

fn search_and_replace(s: &str) {
    assert!(s.starts_with("The quick"));
    assert!(s.contains("fox"));
    assert!(s.ends_with("lazy dog"));
    assert_eq!(s.find('e'), Some(2)); // first match
    assert_eq!(s.rfind('e'), Some(33)); // last match

    let s2 = "the dog and the fox";
    assert_eq!(s2.replace("the", "that"), "that dog and that fox"); // replace all matches
    assert_eq!(s2.replacen("the", "that", 1), "that dog and the fox"); // replace only the first occurrence
}

fn trim_strip() {
    let hello = "   Hello\tWorld\t  ";
    assert_eq!(hello.trim(), "Hello\tWorld");
    assert_eq!(hello.trim_start(), "Hello\tWorld\t  ");
    assert_eq!(hello.trim_end(), "   Hello\tWorld");
    assert_eq!(hello.trim_matches(|c| " Hello\t".contains(c)), "World");
    assert_eq!(hello.trim().strip_suffix("\tWorld"), Some("Hello"));
}

fn convert() {
    let s = String::from("123");
    assert_eq!(s.parse::<u8>(), Ok(123u8)); // result type needs to implement FromStr
    assert_eq!(String::from_utf8(s.as_bytes().to_vec()), Ok(s)); // convert to vector of bytes then to string back again
}

fn extract(s: &str) {
    assert_eq!(s.chars().nth(2), Some('e')); // get the character at a specific index
    assert_eq!(s.get(4..=8), Some("quick")); // extract a slice
}

fn lengths(s: &str) {
    assert_eq!(s.len(), 43);
    assert!(!s.is_empty());
    assert_eq!(s.lines().count(), 1);
}

fn chars(s: &str) {
    assert!(char::is_uppercase(s.chars().next().unwrap()));
    assert!(char::is_numeric('1'));
    assert!(!char::is_alphanumeric('@'));
    assert!(s.is_ascii());
}

fn transforms() {
    assert_eq!("HELLO".to_lowercase(), "hello");
    assert_eq!("hello".to_uppercase(), "HELLO");
    assert_eq!("-".repeat(3), "---"); // equivalent to Python `"-" * 3`
    assert_eq!(capitalize_ascii("hello"), "Hello");
}

fn capitalize_ascii(s: &str) -> String {
    let mut s = s.to_string();
    if let Some(first) = s.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    s
}

fn multiline_strings() {
    let s = "\
        line 1 \
        line 2 \
        line 3";
    assert_eq!(s, "line 1 line 2 line 3");

    let name = "Riki";
    let s = format!(
        "\
        line 1\n\
        {name}\n\
        line 3"
    );
    assert_eq!(s, "line 1\nRiki\nline 3");

    let s = format!(
        r#"hello {name}
line 2
My name is {name}"#
    );
    assert_eq!(s, "hello Riki\nline 2\nMy name is Riki");

    let s = indoc::indoc! {"
        line 1
        line 2
        line 3
    "};
    assert_eq!(s, "line 1\nline 2\nline 3\n");

    let s = indoc::formatdoc! {"
        line 1
        My name is {name}
        line 3
    "};
    assert_eq!(s, "line 1\nMy name is Riki\nline 3\n");
}

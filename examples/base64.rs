use base64::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = r"foobar!!\$";
    let s_base64 = "Zm9vYmFyISFcJA==";

    assert_eq!(BASE64_STANDARD.encode(s), s_base64);
    assert_eq!(BASE64_STANDARD.decode(s_base64)?, s.as_bytes());

    Ok(())
}

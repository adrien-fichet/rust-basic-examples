// https://doc.rust-lang.org/stable/std/result/index.html

fn main() {
    result_with_retries();
}

fn do_something(number: i32) -> Result<i32, String> {
    if number >= 2 {
        return Ok(number);
    }
    Err("error".to_string())
}

fn result_with_retries() {
    let max_tries = 4;
    let mut num_tries = 0;

    let result = loop {
        let something = do_something(num_tries + 1);
        num_tries += 1;
        match something {
            Ok(x) => break Ok(x),
            Err(e) => {
                if num_tries == max_tries {
                    break Err(e);
                }
            }
        }
    };

    assert_eq!(result, Ok(2));
    assert_eq!(num_tries, 2);
}

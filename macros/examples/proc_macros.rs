// types of procedural macros = derive, attribute, functional
// https://www.freecodecamp.org/news/procedural-macros-in-rust/

use proc_macros_lib::log_duration; // proc macros must be defined in a separate crate configured with `lib.proc-macro = true`

#[log_duration] // attribute macro: will log the duration of the function
fn benchmark() -> u16 {
    let mut counter = 0;
    for _ in 0..u16::MAX {
        counter += 1;
    }
    counter
}

fn main() {
    println!("{}", benchmark());
}

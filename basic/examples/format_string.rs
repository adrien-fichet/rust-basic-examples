// https://doc.rust-lang.org/stable/std/fmt/index.html#syntax

use std::fmt::Display;

fn main() {
    let cat = Cat { name: "Riki", age: 4 };

    println!("{}", cat); // using Display
    println!("{:?}", cat); // using Debug
    println!("{:#?}", cat); // pretty print

    let pi = std::f64::consts::PI;
    println!("{:.2}", pi); // display only 2 decimals
    println!("{:05}", 42); // add up to 5 leading zeros

    println!("Pi is {pi}"); // placeholder with name
    println!("Pi is {value}", value = pi);

    println!("{{ }}"); // curly braces literals

    println!("{value:b} {value:o} {value:x} {value:X}", value = 42); // binary, octal, lowercase and uppercase hexadecimal
}

#[derive(Debug)]
struct Cat {
    name: &'static str,
    age: u32,
}

impl Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The cat named {} is {} year(s) old", self.name, self.age)
    }
}

use chrono::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    for _ in 1..=5 {
        println!("{:?}", Local::now());
        thread::sleep(Duration::from_secs(1));
    }
}

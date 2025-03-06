use humantime::{format_duration, parse_duration};
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

// chrono

fn main() {
    elapsed_time();
    human_time_duration();

    //todo!("date formatting");
    //todo!("date subtraction");
    //todo!("timestamp");
    //todo!("timezones");
}

fn elapsed_time() {
    println!("Start");
    let now = Instant::now();

    let one_second = Duration::from_secs(1);
    sleep(one_second);
    let duration = now.elapsed();
    println!("Time elapsed in seconds: {}", duration.as_secs());

    sleep(one_second); // Duration implements Copy, no need to clone
    let duration = now.elapsed();
    println!("Finished in {} seconds", duration.as_secs());
}

fn human_time_duration() {
    let duration = Duration::from_secs(3600 * 3 + 60 * 20 + 30);
    assert_eq!(format_duration(duration).to_string(), String::from("3h 20m 30s"));
    assert_eq!(parse_duration("3h 20m 30s"), Ok(duration));
}

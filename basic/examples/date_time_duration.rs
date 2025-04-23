use chrono::{DateTime, Local, Utc};
use humantime::{format_duration, parse_duration};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    elapsed_time();
    human_time_duration();
    date_formatting();
    timestamp_datetime_conversions();

    //todo!("date subtraction / time delta");
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

fn date_formatting() {
    let utc_now = Utc::now();
    println!("UTC (RFC 3339): {}", utc_now.to_rfc3339());

    // https://docs.rs/chrono/latest/chrono/format/strftime/index.html#specifiers
    let local_now = Local::now();
    println!("Local (custom format): {}", local_now.format("%Y-%m-%d %H:%M:%S"));
}

fn timestamp_datetime_conversions() {
    // timestamp to datetime
    let dt = DateTime::from_timestamp(0, 0).unwrap();
    assert_eq!(dt.to_rfc2822(), "Thu, 1 Jan 1970 00:00:00 +0000");

    // datetime to timestamp
    let dt = DateTime::parse_from_rfc2822("Thu, 1 Jan 1970 00:00:00 +0000").unwrap();
    assert_eq!(dt.timestamp(), 0);
}

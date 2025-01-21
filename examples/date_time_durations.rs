use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

fn main() {
    elasped_time();
    todo!("date formatting");
    todo!("date substraction");
    todo!("timestamp");
}

fn elasped_time() {
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

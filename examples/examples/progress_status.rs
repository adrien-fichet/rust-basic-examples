// https://docs.rs/indicatif/latest/indicatif/

use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use std::thread::sleep;
use std::time::Duration;

fn spin_countdown() {
    let spinner = ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));
    for i in 0..3 {
        spinner.set_message(format!("Starting in {} seconds...", 3 - i));
        sleep(Duration::from_secs(1));
    }
}

fn progress_bar() {
    let steps = 20;
    let bar = ProgressBar::new(steps);
    bar.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:20.cyan/blue} {msg}").unwrap(),
    );

    for i in 1..=steps {
        sleep(Duration::from_millis(100));
        bar.set_message(format!("Finished task #{}", i));
        bar.inc(1);
    }

    bar.finish_with_message(format!("Done, took {}", HumanDuration(bar.elapsed())));
}

fn main() {
    spin_countdown();
    progress_bar();
}

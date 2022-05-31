use std::{env, thread, time::Duration};

fn main() {
    println!("Tester started.");

    ctrlc::set_handler(|| println!("Tester ignores Ctrl+C signal."))
        .expect("Failed to set Ctrl+C handler in Tester.");

    let seconds = env::args()
        .next_back()
        .unwrap_or_default()
        .parse()
        .unwrap_or_default();

    for remaining_seconds in (1..=seconds).rev() {
        println!("Seconds to finish: {remaining_seconds}.");
        thread::sleep(Duration::from_secs(1));
    }

    println!("Tester finished.");
}

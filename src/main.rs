use std::{env, thread, time::Duration};

fn main() {
    println!("Tester started");

    let seconds = env::args()
        .next_back()
        .unwrap_or_default()
        .parse()
        .unwrap_or_default();

    for remaining_seconds in (1..=seconds).rev() {
        println!("Seconds to finish: {remaining_seconds}");
        thread::sleep(Duration::from_secs(1));
    }

    println!("Tester finished");
}

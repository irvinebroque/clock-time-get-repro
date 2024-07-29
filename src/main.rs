use std::time::Duration;
use wasi::wasi_unstable::{clock_time_get, Clockid, Timestamp};

fn main() {
    // The clock ID for the realtime clock
    let clock_id = Clockid::Realtime;

    // Resolution is in nanoseconds
    let precision = 1_000_000;

    // Get the current time
    let time: Timestamp = clock_time_get(clock_id, precision).expect("clock_time_get failed");

    // Convert the time to seconds and nanoseconds
    let seconds = time / 1_000_000_000;
    let nanoseconds = time % 1_000_000_000;

    println!("Current time: {} seconds and {} nanoseconds since the Unix epoch", seconds, nanoseconds);
}
use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    // Check if a command line argument is provided
    let args: Vec<String> = env::args().collect();

    // Default sleep duration is 5 seconds
    let sleep_duration = if args.len() > 1 {
        // Parse the provided argument as u64
        match args[1].parse::<u64>() {
            Ok(seconds) => Duration::from_secs(seconds),
            Err(_) => {
                eprintln!("Invalid argument. Using default sleep duration (5 seconds).");
                Duration::from_secs(5)
            }
        }
    } else {
        Duration::from_secs(5)
    };

    println!("Sleeping for {} seconds...", sleep_duration.as_secs());

    // Sleep for the specified duration
    thread::sleep(sleep_duration);

    println!("Awake after {} seconds!", sleep_duration.as_secs());
}

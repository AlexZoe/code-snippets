use std::{thread, time::Duration};

use signal_handler::env;

fn main() {
    if let Err(_) = env::Environment::init() {
        println!("Failed to initial environment!");
    }

    while env::Environment::is_running() {
        println!("up and running!");
        thread::sleep(Duration::from_secs(1));
    }

    println!("Exiting");
}

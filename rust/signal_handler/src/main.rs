use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error, thread, time::Duration};

struct Environment {
    running: bool,
    _listener: Option<thread::JoinHandle<()>>,
}

impl Environment {
    pub fn init() -> Result<(), Box<dyn Error>> {
        unsafe {
            if ENV.is_none() {
                let mut signals = Signals::new(&[SIGINT])?;
                ENV = Some(Environment {
                    running: true,
                    _listener: Some(thread::spawn(move || {
                        for sig in signals.forever() {
                            ENV.as_mut().unwrap().running = false;
                            println!("Got signal {:?}", sig);
                        }
                    })),
                });
            }
        }
        Ok(())
    }

    pub fn is_running() -> bool {
        unsafe {
            match ENV {
                Some(_) => return ENV.as_ref().unwrap().running,
                _ => return false
            }
        }
    }
}

static mut ENV: Option<Environment> = None;

fn main() {
    if let Err(_) = Environment::init() {
        println!("Failed to initial environment!");
    }

    while Environment::is_running() {
        println!("up and running!");
        thread::sleep(Duration::from_secs(1));
    }

    println!("Exiting");
}

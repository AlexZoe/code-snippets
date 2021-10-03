use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{error::Error, thread};

pub struct Environment {
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
        false
    }
}

static mut ENV: Option<Environment> = None;

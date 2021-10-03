use once_cell::sync::OnceCell;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{
    error::Error,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

pub struct Environment {
    running: AtomicBool,
    listener: OnceCell<thread::JoinHandle<()>>,
}

impl Environment {
    pub fn init() -> Result<(), Box<dyn Error>> {
        ENV.running.store(true, Ordering::Release);
        ENV.listener.get_or_try_init::<_, Box<dyn Error>>(|| {
            let mut signals = Signals::new(&[SIGINT])?;
            Ok(thread::spawn(move || {
                for sig in signals.forever() {
                    ENV.running.store(false, Ordering::Release);
                    println!("Got signal {:?}", sig);
                }
            }))
        })?;

        Ok(())
    }

    pub fn is_running() -> bool {
        ENV.running.load(Ordering::Acquire)
    }
}

static ENV: Environment = Environment {
    running: AtomicBool::new(false),
    listener: OnceCell::new(),
};

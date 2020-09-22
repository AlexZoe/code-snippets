use std::thread;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};

fn main() {
    channel_test();
    mutex_test();
    mutex_struct();
}


fn channel_test() {
    // multiple producer, single consumer channel
    let (tx, rx) = mpsc::channel();

    // Clone sender for additional thread; has to be cloned before
    // first tx channel is moved to thread
    let tx1 = mpsc::Sender::clone(&tx);

    // all variables used within this thread will be moved to it
    // i.e. ownership is transferred to thread
    thread::spawn(move || {
        let val = String::from("hi");
        // Send string
        tx.send(val).unwrap();
    });

    thread::spawn(move || {
        let val = String::from("hello");
        // Send string
        tx1.send(val).unwrap();
    });

    // receive message from thread (blocks)
    // nonblocking alternative is try_recv
    for received in rx {
        println!("Got: {}", received);
    }
}


fn mutex_test() {
    // wrap i32 value inside a mutex which is inside a atomic reference counter
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}


struct EmbeddedCounter {
    counter: i32,
}


fn mutex_struct() {
    // protect struct by mutex
    let emb_counter = Arc::new(Mutex::new(EmbeddedCounter {
        counter: 0
    }));
    let mut handles = vec![];

    for _ in 0..10 {
        // increase reference count on struct
        let emb_counter = Arc::clone(&emb_counter);
        let handle = thread::spawn(move || {
            // lock mutex and get handle to underlying struct
            let mut num = emb_counter.lock().unwrap();

            // access field in struct, dereferencing is done automatically
            num.counter += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", emb_counter.lock().unwrap().counter);
}

use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    println!("main start");
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        println!("spawn run");
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        // thread::sleep(Duration::from_millis(1200));
        println!("changing started");
        *started = true;
        cvar.notify_one();
        println!("spawn end");
    });
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    println!("main running");
    thread::sleep(Duration::from_millis(1200));
    while !*started {
        println!("started={}", *started);
        started = cvar.wait(started).unwrap();
        println!("started={}", *started);
    }

    println!("started changed");
}

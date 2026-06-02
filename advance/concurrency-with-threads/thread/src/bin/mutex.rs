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
        // 获取锁，wait
        let mut started = lock.lock().unwrap();
        println!("changing started");
        *started = true;
        cvar.notify_one();
        println!("spawn end");
        // 子线程作用域结束，隐式释放锁。
    });
    let (lock, cvar) = &*pair;
    // 获取锁
    let mut started = lock.lock().unwrap();
    println!("main running");
    // 虚假唤醒（Spurious Wakeup）: 操作系统底层实现中，线程有时会没有任何 notify 信号的情况下被意外唤醒。
    while !*started {
        println!("started={}", *started);
        // wait 方法会原子地（atomically） 执行两个操作：
        // 1. 释放 Mutex 锁。
        // 2. 将当前线程挂起，等待被唤醒。
        started = cvar.wait(started).unwrap();
        println!("started={}", *started);
    }

    println!("started changed");
}

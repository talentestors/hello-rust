use std::sync::Arc;
use std::thread;

fn main() {
    let s = Arc::new(String::from("多线程漫游者"));
    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || println!("{}", s));
    }
}

// use std::rc::Rc;
// use std::thread;

// fn main() {
//     let s = Rc::new(String::from("多线程漫游者"));
//     for _ in 0..10 {
//         let s = Rc::clone(&s);
//         // 表面原因是 Rc<T> 不能在线程间安全的传递，实际上是因为它没有实现 Send 特征，而该特征恰恰是多线程间传递数据的关键
//         let handle = thread::spawn(move || println!("{}", s));
//     }
// }

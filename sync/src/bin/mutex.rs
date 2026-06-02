use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
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

// use std::rc::Rc;
// use std::sync::Mutex;
// use std::thread;

// fn main() {
//     // 通过`Rc`实现`Mutex`的多所有权
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];

//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         // 创建子线程，并将`Mutex`的所有权拷贝传入到子线程中
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();

//             *num += 1;
//         });
//         handles.push(handle);
//     }

//     // 等待所有子线程完成
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     // 输出最终的计数结果
//     println!("Result: {}", *counter.lock().unwrap());
// }

// use std::sync::Mutex;

// fn main() {
//     // 使用`Mutex`结构体的关联函数创建新的互斥锁实例
//     let m = Mutex::new(5);

//     {
//         // 获取锁，然后deref为`m`的引用
//         // lock返回的是Result
//         let mut num = m.lock().unwrap();
//         *num = 6;
//         // 锁自动被drop
//     }

//     println!("m = {:#?}", m);
// }

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    let mut num = m.lock().unwrap();
    *num = 6;
    // 锁还没有被 drop 就尝试申请下一个锁，导致主线程阻塞
    drop(num); // 手动 drop num ，可以让 num1 申请到下个锁
    let mut num1 = m.lock().unwrap();
    *num1 = 7;
    // drop(num1); // 手动 drop num1 ，观察打印结果的不同

    println!("m = {:?}", m);
}

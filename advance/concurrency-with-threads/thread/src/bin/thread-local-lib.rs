fn main() {
    use std::cell::Cell;
    use std::sync::Arc;
    use std::thread;
    use thread_local::ThreadLocal;

    let tls = Arc::new(ThreadLocal::new());
    let mut v = vec![];
    // 创建多个线程
    for _ in 0..5 {
        let tls2 = tls.clone();
        let handle = thread::spawn(move || {
            // 将计数器加1
            // 请注意，由于线程 ID 在线程退出时会被回收，因此一个线程有可能回收另一个线程的对象
            // 这只能在线程退出后发生，因此不会导致任何竞争条件
            let cell = tls2.get_or(|| Cell::new(0));
            cell.set(cell.get() + 1);
        });
        v.push(handle);
    }
    for handle in v {
        handle.join().unwrap();
    }
    // 一旦所有子线程结束，收集它们的线程局部变量中的计数器值，然后进行求和
    let tls = Arc::try_unwrap(tls).unwrap();
    let total = tls.into_iter().fold(0, |x, y| {
        // 打印每个线程局部变量中的计数器值，发现不一定有5个线程，
        // 因为一些线程已退出，并且其他线程会回收退出线程的对象
        println!("x: {}, y: {}", x, y.get());
        x + y.get()
    });

    // 和为5
    assert_eq!(total, 5);
}

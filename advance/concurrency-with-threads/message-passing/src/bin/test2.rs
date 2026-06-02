use std::sync::mpsc;
fn main() {
    use std::thread;

    // send 需要main结束才会被drop
    let (send, recv) = mpsc::channel();
    let num_threads = 3;
    for i in 0..num_threads {
        let thread_send = send.clone();
        thread::spawn(move || {
            thread_send.send(i).unwrap();
            println!("thread {:?} finished", i);
        });
    }

    // 在这里drop send...
    drop(send);

    for x in recv {
        println!("Got: {}", x);
    }
    println!("finished iterating");
}

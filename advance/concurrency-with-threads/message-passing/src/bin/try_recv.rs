use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
    println!("receive {:?}", rx.try_recv());
}
/*
receive Err(Empty)
receive Ok(1)
receive Err(Disconnected)
receive Err(Disconnected)
receive Err(Disconnected)
*/

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("我，飞走咯!");
        tx.send(s.clone()).unwrap();
        // tx.send(s).unwrap(); // owner move!
        println!("val is {}", s);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

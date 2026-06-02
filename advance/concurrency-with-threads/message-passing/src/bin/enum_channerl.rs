use std::sync::mpsc::{self, Receiver, Sender};

// Tips: Rust 会按照枚举中占用内存最大的那个成员进行内存对齐
enum Fruit {
    Apple(u8),
    Orange(String),
}

fn main() {
    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("received {} apples", count),
            Fruit::Orange(flavor) => println!("received {} oranges", flavor),
        }
    }
}

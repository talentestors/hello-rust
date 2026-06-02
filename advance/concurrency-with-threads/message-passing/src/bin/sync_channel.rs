use std::sync::mpsc;
use std::thread;
use std::time::Duration;
fn main() {
    // mpsc::sync_channel(0): bound=0: 指定同步通道的消息缓存条数
    // 当消息缓冲队列满了后，新的消息发送将被阻塞
    // 而异步通道的缓冲上限取决于你的内存大小
    let (tx, rx) = mpsc::sync_channel(0);

    let handle = thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后");
    });

    println!("睡眠之前");
    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("receive {}", rx.recv().unwrap());
    handle.join().unwrap();
}

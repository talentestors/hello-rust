use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

#[derive(Debug)]
#[allow(unused)]
struct MyBox(*const i32);
unsafe impl Send for MyBox {}
unsafe impl Sync for MyBox {}

impl MyBox {
    fn ptr(&self) -> *const i32 {
        self.0
    }
}

fn main() {
    let x = 5;
    let b = MyBox(&x as *const i32);
    let v = Arc::new(Mutex::new(b));
    let t = thread::spawn(move || unsafe {
        let v1 = v.lock().unwrap().ptr();
        println!("{}", *v1);
    });

    t.join().unwrap();
}

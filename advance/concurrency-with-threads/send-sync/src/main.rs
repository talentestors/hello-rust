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
// use std::sync::Arc;
// use std::thread;

// fn main() {
//     let x = 5;
//     let v = Arc::new(&x as *const i32 as usize);
//     let t = thread::spawn(move || {
//         let v1 = unsafe { (*v as *const i32).read() };
//         println!("{}", v1);
//     });

//     t.join().unwrap();
//     println!("{}", x);
// }

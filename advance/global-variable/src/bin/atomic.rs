use std::sync::atomic::{AtomicUsize, Ordering};
static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);

fn main() {
    for _ in 0..100 {
        REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
    }

    println!("当前用户请求数{:?}", REQUEST_RECV);
}

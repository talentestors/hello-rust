fn main() {
    let a = 1;
    #[allow(unused_mut)]
    let mut b: usize = &a as *const i32 as usize;
    let c: *const i32 = &a;
    unsafe {
        *(b as *mut i32) += 1;
        println!("{}", *c);
    }
}

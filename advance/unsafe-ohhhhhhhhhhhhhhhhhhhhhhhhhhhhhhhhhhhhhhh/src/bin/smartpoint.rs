fn main() {
    let a: Box<i32> = Box::new(10);
    // 需要先解引用a
    let b: *const i32 = &*a;
    // 使用 into_raw 来创建
    // let c: *const i32 = Box::into_raw(a);
    let c: *mut i32 = Box::into_raw(a);

    unsafe {
        *c += 1;
        assert_eq!(*b, *c);
    }
}

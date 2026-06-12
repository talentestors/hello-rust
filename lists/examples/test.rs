fn main() {
    let mut x = 42;

    let reference: &i32 = &x;
    let mutable_ptr: *mut i32 = reference as *const i32 as *mut i32;

    // 尝试通过裸指针修改不可变引用的数据，这是 UB 行为
    unsafe {
        // *mutable_ptr = 10;
    }

    println!("Modified value: {}", reference);
}

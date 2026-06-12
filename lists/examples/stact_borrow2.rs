fn main() {
    unsafe {
        let mut data = [0; 10];
        let ref1_at_0 = &mut data[0]; // 获取第 1 个元素的引用
        let ptr2_at_0 = ref1_at_0 as *mut i32; // 裸指针 ptr 指向第 1 个元素
        let ptr3_at_1 = ptr2_at_0.add(1); // 对裸指针进行运算，指向第 2 个元素

        *ptr3_at_1 += 3;
        *ptr2_at_0 += 2;
        *ref1_at_0 += 1;

        // Should be [3, 3, 0, ...]
        println!("{:?}", &data[..]);
    }
}

// struct SelfRef {
//     data: String,
//     data_ptr: *const String, // 一个裸指针，用来存 data 的地址
// }

// fn main() {
//     // 1. 在栈上创建一个实例 s1
//     let mut s1 = SelfRef {
//         data: String::from("Hello"),
//         data_ptr: std::ptr::null(),
//     };

//     // 2. 建立“自引用”：让 data_ptr 指向 s1 自己的 data 字段
//     s1.data_ptr = &s1.data as *const String;

//     // 3. 灾难发生：移动操作！
//     let s2 = s1;
// }

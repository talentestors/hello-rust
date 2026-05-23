fn main() {
    let mut v = Vec::with_capacity(10);
    v.extend([1, 2, 3]); // 附加数据到 v
    println!("Vector 长度是: {}, 容量是: {}", v.len(), v.capacity());

    v.reserve(100); // 调整 v 的容量，至少要有 100 的容量
    println!(
        "Vector（reserve） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );

    v.shrink_to_fit(); // 释放剩余的容量，一般情况下，不会主动去释放容量
    println!(
        "Vector（shrink_to_fit） 长度是: {}, 容量是: {}",
        v.len(),
        v.capacity()
    );
}

// trait IpAddr {
//     fn display(&self);
// }

// struct V4(String);
// impl IpAddr for V4 {
//     fn display(&self) {
//         println!("ipv4: {:?}", self.0)
//     }
// }
// struct V6(String);
// impl IpAddr for V6 {
//     fn display(&self) {
//         println!("ipv6: {:?}", self.0)
//     }
// }

// fn main() {
//     let v: Vec<Box<dyn IpAddr>> = vec![
//         Box::new(V4("127.0.0.1".to_string())),
//         Box::new(V6("::1".to_string())),
//     ];

//     for ip in v {
//         ip.display();
//     }

//     // 初始化 vec 的更多方式
//     let v = vec![0; 3]; // 默认值为 0，初始长度为 3
//     let v_from = Vec::from([0, 0, 0]);
//     assert_eq!(v, v_from);
// }

// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn main() {
//     let v = vec![
//         IpAddr::V4("127.0.0.1".to_string()),
//         IpAddr::V6("::1".to_string()),
//     ];

//     for ip in v {
//         show_addr(ip)
//     }
// }

// fn show_addr(ip: IpAddr) {
//     println!("{:?}", ip);
// }

// fn main() {
//     // 1
//     let v: Vec<i32> = Vec::new();
//     println!("{:?}", v);
//     // 2 与其它类型一样，必须将 v 声明为 mut 后，才能进行修改。
//     let mut v = Vec::new();
//     v.push(1);
//     println!("{:?}", v);
//     // 3
//     let v = vec![1, 2, 3];
//     println!("{:?}", v);

//     // read
//     let mut v = vec![1, 2, 3, 4, 5];

//     let third: &i32 = &v[2];
//     println!("第三个元素是 {}", third);

//     match v.get(2) {
//         Some(third) => println!("第三个元素是 {third}"),
//         None => println!("去你的第三个元素，根本没有！"),
//     }

//     for i in &mut v {
//         *i += 10;
//     }
//     println!("{:?}", v)
// }

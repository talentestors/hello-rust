struct Buffer<const N: usize> {
    data: [u8; N],
}

const fn compute_buffer_size(factor: usize) -> usize {
    factor * 1024
}

fn main() {
    const SIZE: usize = compute_buffer_size(4);
    let buffer = Buffer::<SIZE> { data: [0; SIZE] };
    println!("Buffer size: {} bytes", buffer.data.len());
}
// fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     a + b
// }
// use std::fmt::Display;

// fn create_and_print<T>()
// where
//     T: From<i32> + Display,
// {
//     let a: T = 100.into(); // 创建了类型为 T 的变量 a，它的初始值由 100 转换而来
//     println!("a is: {}", a);
// }

// struct Point<T> {
//     x: T,
//     #[allow(unused)]
//     y: T,
// }

// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// fn main() {
//     println!("add i8: {}", add(2i8, 3i8));
//     println!("add i32: {}", add(20, 30));
//     println!("add f64: {}", add(1.23, 1.23));

//     create_and_print::<i64>();

//     let p = Point { x: 5, y: 10 };

//     println!("p.x = {}", p.x());
// }

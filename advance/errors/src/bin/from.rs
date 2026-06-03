use std::fs::File;
use std::io::{self, Read};
use std::num;

#[derive(Debug)]
struct AppError {
    kind: String,
    message: String,
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

impl From<num::ParseIntError> for AppError {
    fn from(error: num::ParseIntError) -> Self {
        AppError {
            kind: String::from("parse"),
            message: error.to_string(),
        }
    }
}

fn main() -> Result<(), AppError> {
    let mut file = File::open("hello_world.txt")?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let _number: usize;
    _number = content.parse()?;

    Ok(())
}

// --------------- 上述代码运行后的可能输出 ---------------

// 01. 若 hello_world.txt 文件不存在
// Error: AppError { kind: "io", message: "No such file or directory (os error 2)" }

// 02. 若用户没有相关的权限访问 hello_world.txt
// Error: AppError { kind: "io", message: "Permission denied (os error 13)" }

// 03. 若 hello_world.txt 包含有非数字的内容，例如 Hello, world!
// Error: AppError { kind: "parse", message: "invalid digit found in string" }
// use std::fs::File;
// use std::io;

// #[derive(Debug)]
// struct AppError {
//     kind: String,    // 错误类型
//     message: String, // 错误信息
// }

// // 为 AppError 实现 std::convert::From 特征，由于 From 包含在 std::prelude 中，因此可以直接简化引入。
// // 实现 From<io::Error> 意味着我们可以将 io::Error 错误转换成自定义的 AppError 错误
// impl From<io::Error> for AppError {
//     fn from(error: io::Error) -> Self {
//         AppError {
//             kind: String::from("io"),
//             message: error.to_string(),
//         }
//     }
// }

// fn main() -> Result<(), AppError> {
//     let _file = File::open("nonexistent_file.txt")?;

//     Ok(())
// }

// // --------------- 上述代码运行后输出 ---------------
// // Error: AppError { kind: "io", message: "No such file or directory (os error 2)" }

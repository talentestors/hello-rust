// #[derive(Debug)]
// struct Person {
//     name: String,
//     age: u8,
// }

// fn main() {
//     let i = 3.1415926;
//     let s = String::from("hello");
//     let v = vec![1, 2, 3];
//     let p = Person {
//         name: "sunface".to_string(),
//         age: 18,
//     };
//     /*
//         // {:?}
//     [1, 2, 3], Person { name: "sunface", age: 18 }

//     // {:#?}
//     [
//         1,
//         2,
//         3,
//     ], Person {
//         name: "sunface",
//     }
//     */
//     println!("{}, {}, {}, {}", i, s, v, p);
// }
struct Person {
    name: String,
    age: u8,
}

use std::fmt;
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "大佬在上，请受我一拜，小弟姓名{}，年芳{}，家里无田又无车，生活苦哈哈",
            self.name, self.age
        )
    }
}

fn main() {
    let p = Person {
        name: "sunface".to_string(),
        age: 18,
    };
    println!("{}", p);
}

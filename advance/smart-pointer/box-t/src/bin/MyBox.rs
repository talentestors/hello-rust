use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// fn main() {
//     let y = MyBox::new(5);

//     assert_eq!(5, *y);
// }

fn main() {
    let s: MyBox<String> = MyBox::new(String::from("hello world"));
    display(&s)
}

fn display(s: &str) {
    println!("{}", s);
}

// fn main() {
//     let s = String::from("hello world");
//     display(&s)
// }

// fn display(s: &str) {
//     println!("{}", s);
// }

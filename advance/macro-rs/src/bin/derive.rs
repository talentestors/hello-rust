use hello_macro_derive::MyDefault;

#[derive(MyDefault, Debug)]
struct SomeData(u32, String);

#[derive(MyDefault, Debug)]
struct User {
    name: String,
    data: SomeData,
}

fn main() {
    println!("{:?}", User::default());
}

// use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;

// #[derive(HelloMacro)]
// struct Sunfei;

// #[derive(HelloMacro)]
// struct Sunface;

// fn main() {
//     Sunfei::hello_macro();
//     Sunface::hello_macro();
// }

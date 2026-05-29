fn get_person() -> String {
    String::from("sunface")
}
fn main() {
    let p = get_person();
    println!("Hello, {}!", p); // implicit position
    println!("Hello, {0}!", p); // explicit index
    println!("Hello, {person}!", person = p);

    // 在格式化字符串时捕获环境中的值（Rust 1.58 新增）
    let person = get_person();
    println!("Hello, {person}!");
}

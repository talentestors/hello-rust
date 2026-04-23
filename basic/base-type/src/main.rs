fn main() {
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b); // 19
    println!("{}", 0.1 + 0.2);
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
}

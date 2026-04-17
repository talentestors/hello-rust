use utf8_slice;

fn main() {
    let s = String::from("hello,world!");
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    let s = "holla中国人नमस्ते";
    s.chars().for_each(|c| print!("{}", c));
    println!();
    println!("length: {}", utf8_slice::len(s));
}

fn say_hello(s: &str) {
    println!("{}", s);
}

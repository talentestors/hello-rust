fn main() {
    let _s1: &str = "banana";
    let _s2: &str = &String::from("banana");

    let arr = [1, 2, 3, 4, 5];

    let _s3: &[i32] = &arr[1..3];
}

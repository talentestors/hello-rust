struct Array(Vec<i32>);

use std::fmt;
impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "数组是：{:?}", self.0)
    }
}

fn main() {
    let arr = Array(vec![1, 2, 3]);
    println!("{}", arr);
}

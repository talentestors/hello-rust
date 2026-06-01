// use std::convert::TryInto;

fn main() {
    let a: u8 = 10;
    let b: i16 = 1500;

    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
    if a > b_ {
        println!("Ten is less than one hundred.");
    }
}
// fn main() {
//     let a: u8 = 10;
//     let b: u16 = 150;

//     let b_: u8 = b.try_into().unwrap();

//     if a < b_ {
//         println!("Ten is less than one hundred.");
//     }
// }

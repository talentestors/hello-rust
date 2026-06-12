fn main() {
    unsafe {
        let mut data = Box::new(10);
        let ptr1 = (&mut *data) as *mut i32;

        *data += 10;
        *ptr1 += 1;

        // Should be 21
        println!("{}", data);
    }
}
/*
$ cargo run

21

MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo +nightly-2022-01-21 miri run

error: Undefined Behavior: no item granting read access
       to tag <1707> at alloc763 found in borrow stack.

 --> src\main.rs:7:5
  |
7 |     *ptr1 += 1;
  |     ^^^^^^^^^^ no item granting read access to tag <1707>
  |                at alloc763 found in borrow stack.
  |
*/

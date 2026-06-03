static mut REQUEST_RECV: usize = 0;

fn main() {
    #[allow(static_mut_refs)]
    unsafe {
        REQUEST_RECV += 1;
        assert_eq!(REQUEST_RECV, 1);
        println!("{}", REQUEST_RECV);
    }
}

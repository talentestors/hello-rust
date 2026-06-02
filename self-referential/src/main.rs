#[derive(Debug)]
struct SelfRef {
    value: String,
    pointer_to_value: *const String,
}

impl SelfRef {
    fn new(txt: &str) -> Self {
        SelfRef {
            value: String::from(txt),
            pointer_to_value: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.value;
        self.pointer_to_value = self_ref;
    }

    fn value(&self) -> &str {
        &self.value
    }

    fn pointer_to_value(&self) -> &String {
        assert!(
            !self.pointer_to_value.is_null(),
            "SelfRef::pointer_to_value called without SelfRef::init being called first"
        );
        unsafe { &*(self.pointer_to_value) }
    }
}

fn main() {
    let mut t = SelfRef::new("hello");
    t.init();
    // 打印值和指针地址
    println!("{}, {:p}", t.value(), t.pointer_to_value());
}

#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

fn main() {
    let u = MyUnion { f2: 1.7 };
    let f = unsafe { u.f2 };
    let f1 = unsafe { u.f1 };
    println!("f1={f1}\nf2={f}");
}

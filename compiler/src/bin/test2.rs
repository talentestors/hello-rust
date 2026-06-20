use std::cell::RefCell;
use std::rc::Rc;

pub struct Foo {
    pub foo1: Vec<bool>,
    pub foo2: Vec<i32>,
}
fn main() {
    let foo_cell = Rc::new(RefCell::new(Foo {
        foo1: vec![true, false],
        foo2: vec![1, 2],
    }));

    let borrow = foo_cell.borrow_mut();
    let foo1 = &borrow.foo1;
    // 下面代码会报错,因为`foo1`和`foo2`发生了重复借用
    borrow.foo2.iter_mut().enumerate().for_each(|(idx, foo2)| {
        if foo1[idx] {
            *foo2 *= -1;
        }
    });
}

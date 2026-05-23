# 下标索引与 .get 的区别

这两种方式都能成功的读取到指定的数组元素，既然如此为什么会存在两种方法？何况 .get 还会增加使用复杂度，这就涉及到数组越界的问题了，让我们通过示例说明：

```rust
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```

运行以上代码，`&v[100]` 的访问方式会导致程序无情报错退出，因为发生了数组越界访问。 但是 `v.get` 就不会，它在内部做了处理，有值的时候返回 `Some(T)`，无值的时候返回 `None`，因此 `v.get` 的使用方式非常安全。

既然如此，为何不统一使用 `v.get` 的形式？因为实在是有些啰嗦，Rust 语言的设计者和使用者在审美这方面还是相当统一的：简洁即正义，何况性能上也会有轻微的损耗。

既然有两个选择，肯定就有如何选择的问题，答案很简单，当你确保索引不会越界的时候，就用索引访问，否则用 `.get`。例如，访问第几个数组元素并不取决于我们，而是取决于用户的输入时，用 `.get` 会非常适合，天知道那些可爱的用户会输入一个什么样的数字进来！

# Vector 常见的一些方法示例

```rust
let mut v =  vec![1, 2];
assert!(!v.is_empty());         // 检查 v 是否为空

v.insert(2, 3);                 // 在指定索引插入数据，索引值不能大于 v 的长度， v: [1, 2, 3] 
assert_eq!(v.remove(1), 2);     // 移除指定位置的元素并返回, v: [1, 3]
assert_eq!(v.pop(), Some(3));   // 删除并返回 v 尾部的元素，v: [1]
assert_eq!(v.pop(), Some(1));   // v: []
assert_eq!(v.pop(), None);      // 记得 pop 方法返回的是 Option 枚举值
v.clear();                      // 清空 v, v: []

let mut v1 = [11, 22].to_vec(); // append 操作会导致 v1 清空数据，增加可变声明
v.append(&mut v1);              // 将 v1 中的所有元素附加到 v 中, v1: []
v.truncate(1);                  // 截断到指定长度，多余的元素被删除, v: [11]
v.retain(|x| *x > 10);          // 保留满足条件的元素，即删除不满足条件的元素

let mut v = vec![11, 22, 33, 44, 55];
// 删除指定范围的元素，同时获取被删除元素的迭代器, v: [11, 55], m: [22, 33, 44]
let mut m: Vec<_> = v.drain(1..=3).collect();    

let v2 = m.split_off(1);        // 指定索引处切分成两个 vec, m: [22], v2: [33, 44]
```

数组切片的方式获取 vec 的部分元素

```rust
fn main() {
    let v = vec![11, 22, 33, 44, 55];
    let slice = &v[1..=3];
    assert_eq!(slice, &[22, 33, 44]);
}
```

# 排序

整数数组的排序
以下是对整数列进行排序的例子。

```rust
fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];    
    vec.sort_unstable();    
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}
```
浮点数数组的排序
我们尝试使用上面的方法来对浮点数进行排序：

``
fn main() {
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];    
    vec.sort_unstable();    
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}
```

结果，居然报错了，

```
error[E0277]: the trait bound `f32: Ord` is not satisfied
    --> src/main.rs:29:13
     |
29   |         vec.sort_unstable();
     |             ^^^^^^^^^^^^^ the trait `Ord` is not implemented for `f32`
     |
     = help: the following other types implement trait `Ord`:
               i128
               i16
               i32
               i64
               i8
               isize
               u128
               u16
             and 4 others
note: required by a bound in `core::slice::::sort_unstable`
    --> /home/keijack/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/mod.rs:2635:12
     |
2635 |         T: Ord,
     |            ^^^ required by this bound in `core::slice::::sort_unstable`

For more information about this error, try `rustc --explain E0277`.
```

原来，在浮点数当中，存在一个 NAN 的值，这个值无法与其他的浮点数进行对比，因此，浮点数类型并没有实现全数值可比较 Ord 的特性，而是实现了部分可比较的特性 PartialOrd。

如此，如果我们确定在我们的浮点数数组当中，不包含 NAN 值，那么我们可以使用 partial_cmp 来作为大小判断的依据。

```rust
fn main() {
    let mut vec = vec![1.0, 5.6, 10.3, 2.0, 15f32];    
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());    
    assert_eq!(vec, vec![1.0, 2.0, 5.6, 10.3, 15f32]);
}
```

OK，现在可以正确执行了。

对结构体数组进行排序
有了上述浮点数排序的经验，我们推而广之，那么对结构体是否也可以使用这种自定义对比函数的方式来进行呢？马上来试一下：

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    // 定义一个按照年龄倒序排序的对比函数
    people.sort_unstable_by(|a, b| b.age.cmp(&a.age));

    println!("{:?}", people);
}
```

执行后输出：

```
[Person { name: "Al", age: 60 }, Person { name: "Zoe", age: 25 }, Person { name: "John", age: 1 }]
```

结果正确。

从上面我们学习过程当中，排序需要我们实现 Ord 特性，那么如果我们把我们的结构体实现了该特性，是否就不需要我们自定义对比函数了呢？

是，但不完全是，实现 Ord 需要我们实现 Ord、Eq、PartialEq、PartialOrd 这些属性。好消息是，你可以 derive 这些属性：

```rust
#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("Al".to_string(), 30),
        Person::new("John".to_string(), 1),
        Person::new("John".to_string(), 25),
    ];

    people.sort_unstable();

    println!("{:?}", people);
}
```

执行输出

```
[Person { name: "Al", age: 30 }, Person { name: "Al", age: 60 }, Person { name: "John", age: 1 }, Person { name: "John", age: 25 }, Person { name: "Zoe", age: 25 }]
```

需要 derive Ord 相关特性，需要确保你的结构体中所有的属性均实现了 Ord 相关特性，否则会发生编译错误。derive 的默认实现会依据属性的顺序依次进行比较，如上述例子中，当 Person 的 name 值相同，则会使用 age 进行比较。

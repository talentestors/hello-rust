## 三种 Fn 特征

闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：转移所有权、可变借用、不可变借用，因此相应的 `Fn` 特征也有三种：

1. `FnOnce`，该类型的闭包会拿走被捕获变量的所有权。`Once` 顾名思义，说明该闭包只能运行一次：

```rust
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}
```

**仅**实现 `FnOnce` 特征的闭包在调用时会转移所有权，所以显然不能对已失去所有权的闭包变量进行二次调用：

```console
error[E0382]: use of moved value: `func`
 --> src\main.rs:6:20
  |
1 | fn fn_once<F>(func: F)
  |               ---- move occurs because `func` has type `F`, which does not implement the `Copy` trait
                  // 因为`func`的类型是没有实现`Copy`特性的 `F`，所以发生了所有权的转移
...
5 |     println!("{}", func(3));
  |                    ------- `func` moved due to this call // 转移在这
6 |     println!("{}", func(4));
  |                    ^^^^ value used here after move // 转移后再次用
  |
```

这里面有一个很重要的提示，因为 `F` 没有实现 `Copy` 特征，所以会报错，那么我们添加一个约束，试试实现了 `Copy` 的闭包：

```rust
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy,// 改动在这里
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}
```

上面代码中，`func` 的类型 `F` 实现了 `Copy` 特征，调用时使用的将是它的拷贝，所以并没有发生所有权的转移。

```console
true
false
```

如果你想强制闭包取得捕获变量的所有权，可以在参数列表前添加 `move` 关键字，这种用法通常用于闭包的生命周期大于捕获变量的生命周期时，例如将闭包返回或移入其他线程。

```rust
use std::thread;
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});
handle.join().unwrap();
```

2. `FnMut`，它以可变借用的方式捕获了环境中的值，因此可以修改该值：

```rust
fn main() {
    let mut s = String::new();

    let update_string =  |str| s.push_str(str);
    update_string("hello");

    println!("{:?}",s);
}
```

在闭包中，我们调用 `s.push_str` 去改变外部 `s` 的字符串值，因此这里捕获了它的可变借用，运行下试试：

```console
error[E0596]: cannot borrow `update_string` as mutable, as it is not declared as mutable
 --> src/main.rs:5:5
  |
4 |     let update_string =  |str| s.push_str(str);
  |         -------------          - calling `update_string` requires mutable binding due to mutable borrow of `s`
  |         |
  |         help: consider changing this to be mutable: `mut update_string`
5 |     update_string("hello");
  |     ^^^^^^^^^^^^^ cannot borrow as mutable
```

虽然报错了，但是编译器给出了非常清晰的提示，想要在闭包内部捕获可变借用，需要把该闭包声明为可变类型，也就是 `update_string` 要修改为 `mut update_string`：

```rust
fn main() {
    let mut s = String::new();

    let mut update_string =  |str| s.push_str(str);
    update_string("hello");

    println!("{:?}",s);
}
```

这种写法有点反直觉，相比起来前面的 `move` 更符合使用和阅读习惯。但是如果你忽略 `update_string` 的类型，仅仅把它当成一个普通变量，那么这种声明就比较合理了。

再来看一个复杂点的：

```rust
fn main() {
    let mut s = String::new();

    let update_string =  |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

fn exec<'a, F: FnMut(&'a str)>(mut f: F)  {
    f("hello")
}
```

这段代码中`update_string`没有使用mut关键字修饰，而上文提到想要在闭包内部捕获可变借用，需要用关键词把该闭包声明为可变类型。我们确实这么做了———`exec(mut f: F)`表明我们的`exec`接收的是一个可变类型的闭包。这段代码中`update_string`看似被声明为不可变闭包，但是`exec(mut f: F)`函数接收的又是可变参数，为什么可以正常执行呢？

rust不可能接受类型不匹配的形参和实参通过编译，我们提供的实参又是可变的，这说明`update_string`一定是一个可变类型的闭包，我们不妨看看rust-analyzer自动给出的类型标注：

```rust
    let mut s: String = String::new();

    let update_string: impl FnMut(&str) =  |str| s.push_str(str);
```

rust-analyzer给出的类型标注非常清晰的说明了 `update_string` 实现了 `FnMut` 特征。

为什么`update_string`没有用`mut`修饰却是一个可变类型的闭包？事实上，`FnMut`只是trait的名字，声明变量为`FnMut`和要不要mut没啥关系，`FnMut`是推导出的特征类型，`mut`是rust语言层面的一个修饰符，用于声明一个绑定是可变的。Rust从特征类型系统和语言修饰符两方面保障了我们的程序正确运行。

我们在使用`FnMut`类型闭包时需要捕获外界的可变借用，因此我们常常搭配`mut`修饰符使用。但我们要始终记住，二者是相互独立的。

因此，让我们再回头分析一下这段代码：在`main`函数中，首先创建了一个可变的字符串`s`，然后定义了一个可变类型闭包`update_string`，该闭包接受一个字符串参数并将其追加到`s`中。接下来调用了`exec`函数，并将`update_string`闭包的所有权移交给它。最后打印出了字符串`s`的内容。

细心的读者可能注意到，我们在上文的分析中提到`update_string`闭包的所有权被移交给了`exec`函数。这说明`update_string`没有实现`Copy`特征，但并不是所有闭包都没有实现`Copy`特征，闭包自动实现`Copy`特征的规则是，只要闭包捕获的类型都实现了`Copy`特征的话，这个闭包就会默认实现`Copy`特征。

我们来看一个例子：

```rust
let s = String::new();
let update_string =  || println!("{}",s);
```

这里取得的是`s`的不可变引用，所以是能`Copy`的。而如果拿到的是`s`的所有权或可变引用，都是不能`Copy`的。我们刚刚的代码就属于第二类，取得的是`s`的可变引用，没有实现`Copy`。

```rust
// 拿所有权
let s = String::new();
let update_string = move || println!("{}", s);

exec(update_string);
// exec2(update_string); // 不能再用了

// 可变引用
let mut s = String::new();
let mut update_string = || s.push_str("hello");
exec(update_string);
// exec1(update_string); // 不能再用了
```

3. `Fn` 特征，它以不可变借用的方式捕获环境中的值
   让我们把上面的代码中 `exec` 的 `F` 泛型参数类型修改为 `Fn(&'a str)`：

```rust
fn main() {
    let mut s = String::new();

    let update_string =  |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

fn exec<'a, F: Fn(&'a str)>(mut f: F)  {
    f("hello")
}
```

然后运行看看结果：

```console
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
 --> src/main.rs:4:26  // 期望闭包实现的是`Fn`特征，但是它只实现了`FnMut`特征
  |
4 |     let update_string =  |str| s.push_str(str);
  |                          ^^^^^^-^^^^^^^^^^^^^^
  |                          |     |
  |                          |     closure is `FnMut` because it mutates the variable `s` here
  |                          this closure implements `FnMut`, not `Fn` //闭包实现的是FnMut，而不是Fn
5 |
6 |     exec(update_string);
  |     ---- the requirement to implement `Fn` derives from here
```

从报错中很清晰的看出，我们的闭包实现的是 `FnMut` 特征，需要的是可变借用，但是在 `exec` 中却给它标注了 `Fn` 特征，因此产生了不匹配，再来看看正确的不可变借用方式：

```rust
fn main() {
    let s = "hello, ".to_string();

    let update_string =  |str| println!("{},{}",s,str);

    exec(update_string);

    println!("{:?}",s);
}

fn exec<'a, F: Fn(String) -> ()>(f: F)  {
    f("world".to_string())
}
```

在这里，因为无需改变 `s`，因此闭包中只对 `s` 进行了不可变借用，那么在 `exec` 中，将其标记为 `Fn` 特征就完全正确。

### move 和 Fn

在上面，我们讲到了 `move` 关键字对于 `FnOnce` 特征的重要性，但是实际上使用了 `move` 的闭包依然可以使用 `Fn` 或 `FnMut` 特征。

因为，**一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们**。`move` 本身强调的就是后者，闭包如何捕获变量：

```rust
fn main() {
    let s = String::new();

    let update_string =  move || println!("{}",s);

    exec(update_string);
}

fn exec<F: FnOnce()>(f: F)  {
    f()
}
```

我们在上面的闭包中使用了 `move` 关键字，所以我们的闭包捕获了它，但是由于闭包获取了 `s` 的所有权，因此该闭包实现了 `FnOnce` 的特征。

但是假如我们将代码修改成下面这样，依然可以编译：

```rust
fn main() {
    let s = String::new();

    let update_string =  move || println!("{}",s);

    exec(update_string);
}

fn exec<F: Fn()>(f: F)  {
    f()
}
```

奇怪， 明明是闭包实现的是 `FnOnce` 的特征， 为什么编译器居然允许 `Fn` 特征通过编译呢？

### 三种 Fn 的关系

实际上，一个闭包并不仅仅实现某一种 `Fn` 特征，规则如下：

- 所有的闭包都自动实现了 `FnOnce` 特征，因此任何一个闭包都至少可以被调用一次
- 没有移出所捕获变量的所有权的闭包自动实现了 `FnMut` 特征
- 不需要对捕获变量进行改变的闭包自动实现了 `Fn` 特征

用一段代码来简单诠释上述规则：

```rust
fn main() {
    let s = String::new();

    let update_string =  || println!("{}",s);

    exec(update_string);
    exec1(update_string);
    exec2(update_string);
}

fn exec<F: FnOnce()>(f: F)  {
    f()
}

fn exec1<F: FnMut()>(mut f: F)  {
    f()
}

fn exec2<F: Fn()>(f: F)  {
    f()
}
```

虽然，闭包只是对 `s` 进行了不可变借用，实际上，它可以适用于任何一种 `Fn` 特征：三个 `exec` 函数说明了一切。强烈建议读者亲自动手试试各种情况下使用的 `Fn` 特征，更有助于加深这方面的理解。

关于第二条规则，有如下示例：

```rust
fn main() {
    let mut s = String::new();

    let update_string = |str| -> String {s.push_str(str); s };

    exec(update_string);
}

fn exec<'a, F: FnMut(&'a str) -> String>(mut f: F) {
    f("hello");
}
```

```console
5 |     let update_string = |str| -> String {s.push_str(str); s };
  |                         ^^^^^^^^^^^^^^^                   - closure is `FnOnce` because it moves the variable `s` out of its environment
  |                                                           // 闭包实现了`FnOnce`，因为它从捕获环境中移出了变量`s`
  |                         |
  |                         this closure implements `FnOnce`, not `FnMut`
```

此例中，闭包从捕获环境中移出了变量 `s` 的所有权，因此这个闭包仅自动实现了 `FnOnce`，未实现 `FnMut` 和 `Fn`。再次印证之前讲的**一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量，而不是取决于闭包如何捕获它们**，跟是否使用 `move` 没有必然联系。

如果还是有疑惑？没关系，我们来看看这三个特征的简化版源码：

```rust
pub trait Fn<Args> : FnMut<Args> {
    extern "rust-call" fn call(&self, args: Args) -> Self::Output;
}

pub trait FnMut<Args> : FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}

pub trait FnOnce<Args> {
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
```

看到没？从特征约束能看出来 `Fn` 的前提是实现 `FnMut`，`FnMut` 的前提是实现 `FnOnce`，因此要实现 `Fn` 就要同时实现 `FnMut` 和 `FnOnce`，这段源码从侧面印证了之前规则的正确性。

从源码中还能看出一点：`Fn` 获取 `&self`，`FnMut` 获取 `&mut self`，而 `FnOnce` 获取 `self`。
在实际项目中，**建议先使用 `Fn` 特征**，然后编译器会告诉你正误以及该如何选择。

## 闭包作为函数返回值

看到这里，相信大家对于如何使用闭包作为函数参数，已经很熟悉了，但是如果要使用闭包作为函数返回值，该如何做？

先来看一段代码：

```rust
fn factory() -> Fn(i32) -> i32 {
    let num = 5;

    |x| x + num
}

let f = factory();

let answer = f(1);
assert_eq!(6, answer);
```

上面这段代码看起来还是蛮正常的，用 `Fn(i32) -> i32` 特征来代表 `|x| x + num`，非常合理嘛，肯定可以编译通过, 可惜理想总是难以照进现实，编译器给我们报了一大堆错误，先挑几个重点来看看：

```console
fn factory<T>() -> Fn(i32) -> i32 {
  |                    ^^^^^^^^^^^^^^ doesn't have a size known at compile-time // 该类型在编译器没有固定的大小
```

Rust 要求函数的参数和返回类型，必须有固定的内存大小，例如 `i32` 就是 4 个字节，引用类型是 8 个字节，总之，绝大部分类型都有固定的大小，但是不包括特征，因为特征类似接口，对于编译器来说，无法知道它后面藏的真实类型是什么，因为也无法得知具体的大小。

同样，我们也无法知道闭包的具体类型，该怎么办呢？再看看报错提示：

```console
help: use `impl Fn(i32) -> i32` as the return type, as all return paths are of type `[closure@src/main.rs:11:5: 11:21]`, which implements `Fn(i32) -> i32`
  |
8 | fn factory<T>() -> impl Fn(i32) -> i32 {
```

嗯，编译器提示我们加一个 `impl` 关键字，哦，这样一说，读者可能就想起来了，`impl Trait` 可以用来返回一个实现了指定特征的类型，那么这里 `impl Fn(i32) -> i32` 的返回值形式，说明我们要返回一个闭包类型，它实现了 `Fn(i32) -> i32` 特征。

完美解决，但是，在[特征](https://course.rs/basic/trait/trait.html)那一章，我们提到过，`impl Trait` 的返回方式有一个非常大的局限，就是你只能返回同样的类型，例如：

```rust
fn factory(x:i32) -> impl Fn(i32) -> i32 {

    let num = 5;

    if x > 1{
        move |x| x + num
    } else {
        move |x| x - num
    }
}
```

运行后，编译器报错：

```console
error[E0308]: `if` and `else` have incompatible types
  --> src/main.rs:15:9
   |
12 | /     if x > 1{
13 | |         move |x| x + num
   | |         ---------------- expected because of this
14 | |     } else {
15 | |         move |x| x - num
   | |         ^^^^^^^^^^^^^^^^ expected closure, found a different closure
16 | |     }
   | |_____- `if` and `else` have incompatible types
   |
```

嗯，提示很清晰：`if` 和 `else` 分支中返回了不同的闭包类型，这就很奇怪了，明明这两个闭包长的一样的，好在细心的读者应该回想起来，本章节前面咱们有提到：就算签名一样的闭包，类型也是不同的，因此在这种情况下，就无法再使用 `impl Trait` 的方式去返回闭包。

怎么办？再看看编译器提示，里面有这样一行小字：

```console
= help: consider boxing your closure and/or using it as a trait object
```

哦，相信你已经恍然大悟，可以用特征对象！只需要用 `Box` 的方式即可实现：

```rust
fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}
```

至此，闭包作为函数返回值就已完美解决，若以后你再遇到报错时，一定要仔细阅读编译器的提示，很多时候，转角都能遇到爱。

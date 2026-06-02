## 死锁

在 Rust 中有多种方式可以创建死锁，了解这些方式有助于你提前规避可能的风险，一起来看看。

#### 单线程死锁

这种死锁比较容易规避，但是当代码复杂后还是有可能遇到：

```rust
use std::sync::Mutex;

fn main() {
    let data = Mutex::new(0);
    let d1 = data.lock();
    let d2 = data.lock();
} // d1锁在此处释放
```

非常简单，只要你在另一个锁还未被释放时去申请新的锁，就会触发，当代码复杂后，这种情况可能就没有那么显眼。

#### 多线程死锁

当我们拥有两个锁，且两个线程各自使用了其中一个锁，然后试图去访问另一个锁时，就可能发生死锁：

```rust
use std::{sync::{Mutex, MutexGuard}, thread};
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;
lazy_static! {
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

fn main() {
    // 存放子线程的句柄
    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                // 线程1
                if i_thread % 2 == 0 {
                    // 锁住MUTEX1
                    let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                    // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                    sleep(Duration::from_millis(10));

                    // 去锁MUTEX2
                    let guard = MUTEX2.lock().unwrap();
                // 线程2
                } else {
                    // 锁住MUTEX2
                    let _guard = MUTEX2.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);

                    let _guard = MUTEX1.lock().unwrap();
                }
            }
        }));
    }

    // 等子线程完成
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生");
}
```

在上面的描述中，我们用了"可能"二字，原因在于死锁在这段代码中不是必然发生的，总有一次运行你能看到最后一行打印输出。这是由于子线程的初始化顺序和执行速度并不确定，我们无法确定哪个线程中的锁先被执行，因此也无法确定两个线程对锁的具体使用顺序。

但是，可以简单的说明下死锁发生的必然条件：线程 1 锁住了`MUTEX1`并且线程`2`锁住了`MUTEX2`，然后线程 1 试图去访问`MUTEX2`，同时线程`2`试图去访问`MUTEX1`，就会死锁。 因为线程 2 需要等待线程 1 释放`MUTEX1`后，才会释放`MUTEX2`，而与此同时，线程 1 需要等待线程 2 释放`MUTEX2`后才能释放`MUTEX1`，这种情况造成了两个线程都无法释放对方需要的锁，最终死锁。

那么为何某些时候，死锁不会发生？原因很简单，线程 2 在线程 1 锁`MUTEX1`之前，就已经全部执行完了，随之线程 2 的`MUTEX2`和`MUTEX1`被全部释放，线程 1 对锁的获取将不再有竞争者。 同理，线程 1 若全部被执行完，那线程 2 也不会被锁，因此我们在线程 1 中间加一个睡眠，增加死锁发生的概率。如果你在线程 2 中同样的位置也增加一个睡眠，那死锁将必然发生!

#### try_lock

与`lock`方法不同，`try_lock`会**尝试**去获取一次锁，如果无法获取会返回一个错误，因此**不会发生阻塞**:

```rust
use std::{sync::{Mutex, MutexGuard}, thread};
use std::thread::sleep;
use std::time::Duration;

use lazy_static::lazy_static;
lazy_static! {
    static ref MUTEX1: Mutex<i64> = Mutex::new(0);
    static ref MUTEX2: Mutex<i64> = Mutex::new(0);
}

fn main() {
    // 存放子线程的句柄
    let mut children = vec![];
    for i_thread in 0..2 {
        children.push(thread::spawn(move || {
            for _ in 0..1 {
                // 线程1
                if i_thread % 2 == 0 {
                    // 锁住MUTEX1
                    let guard: MutexGuard<i64> = MUTEX1.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX1，接着准备去锁MUTEX2 !", i_thread);

                    // 当前线程睡眠一小会儿，等待线程2锁住MUTEX2
                    sleep(Duration::from_millis(10));

                    // 去锁MUTEX2
                    let guard = MUTEX2.try_lock();
                    println!("线程 {} 获取 MUTEX2 锁的结果: {:?}", i_thread, guard);
                // 线程2
                } else {
                    // 锁住MUTEX2
                    let _guard = MUTEX2.lock().unwrap();

                    println!("线程 {} 锁住了MUTEX2, 准备去锁MUTEX1", i_thread);
                    sleep(Duration::from_millis(10));
                    let guard = MUTEX1.try_lock();
                    println!("线程 {} 获取 MUTEX1 锁的结果: {:?}", i_thread, guard);
                }
            }
        }));
    }

    // 等子线程完成
    for child in children {
        let _ = child.join();
    }

    println!("死锁没有发生");
}
```

为了演示`try_lock`的作用，我们特定使用了之前必定会死锁的代码，并且将`lock`替换成`try_lock`，与之前的结果不同，这段代码将不会再有死锁发生：

```console
线程 0 锁住了MUTEX1，接着准备去锁MUTEX2 !
线程 1 锁住了MUTEX2, 准备去锁MUTEX1
线程 1 获取 MUTEX1 锁的结果: Err("WouldBlock")
线程 0 获取 MUTEX2 锁的结果: Ok(0)
死锁没有发生
```

如上所示，当`try_lock`失败时，会报出一个错误:`Err("WouldBlock")`，接着线程中的剩余代码会继续执行，不会被阻塞。

> 一个有趣的命名规则：在 Rust 标准库中，使用`try_xxx`都会尝试进行一次操作，如果无法完成，就立即返回，不会发生阻塞。例如消息传递章节中的`try_recv`以及本章节中的`try_lock`

## 内存顺序

内存顺序是指 CPU 在访问内存时的顺序，该顺序可能受以下因素的影响：

- 代码中的先后顺序
- 编译器优化导致在编译阶段发生改变(内存重排序 reordering)
- 运行阶段因 CPU 的缓存机制导致顺序被打乱

#### 编译器优化导致内存顺序的改变

对于第二点，我们举个例子：

```rust
static mut X: u64 = 0;
static mut Y: u64 = 1;

fn main() {
    ...     // A

    unsafe {
        ... // B
        X = 1;
        ... // C
        Y = 3;
        ... // D
        X = 2;
        ... // E
    }
}
```

假如在`C`和`D`代码片段中，根本没有用到`X = 1`，那么编译器很可能会将`X = 1`和`X = 2`进行合并:

```rust
 ...     // A

unsafe {
    ... // B
    X = 2;
    ... // C
    Y = 3;
    ... // D
    ... // E
}
```

若代码`A`中创建了一个新的线程用于读取全局静态变量`X`，则该线程将无法读取到`X = 1`的结果，因为在编译阶段就已经被优化掉。

#### CPU 缓存导致的内存顺序的改变

假设之前的`X = 1`没有被优化掉，并且在代码片段`A`中有一个新的线程:

```console
initial state: X = 0, Y = 1

THREAD Main     THREAD A
X = 1;          if X == 1 {
Y = 3;              Y *= 2;
X = 2;          }
```

我们来讨论下以上线程状态，`Y`最终的可能值(可能性依次降低):

- `Y = 3`: 线程`Main`运行完后才运行线程`A`，或者线程`A`运行完后再运行线程`Main`
- `Y = 6`: 线程`Main`的`Y = 3`运行完，但`X = 2`还没被运行， 此时线程 A 开始运行`Y *= 2`, 最后才运行`Main`线程的`X = 2`
- `Y = 2`: 线程`Main`正在运行`Y = 3`还没结束，此时线程`A`正在运行`Y *= 2`, 因此`Y`取到了值 1，然后`Main`的线程将`Y`设置为 3， 紧接着就被线程`A`的`Y = 2`所覆盖
- `Y = 2`: 上面的还只是一般的数据竞争，这里虽然产生了相同的结果`2`，但是背后的原理大相径庭: 线程`Main`运行完`Y = 3`，但是 CPU 缓存中的`Y = 3`还没有被同步到其它 CPU 缓存中，此时线程`A`中的`Y *= 2`就开始读取`Y`，结果读到了值`1`，最终计算出结果`2`

甚至更改成:

```console
initial state: X = 0, Y = 1

THREAD Main     THREAD A
X = 1;          if X == 2 {
Y = 3;              Y *= 2;
X = 2;          }
```

还是可能出现`Y = 2`，因为`Main`线程中的`X`和`Y`被同步到其它 CPU 缓存中的顺序未必一致。

#### 限定内存顺序的 5 个规则

在理解了内存顺序可能存在的改变后，你就可以明白为什么 Rust 提供了`Ordering::Relaxed`用于限定内存顺序了，事实上，该枚举有 5 个成员:

- **Relaxed**， 这是最宽松的规则，它对编译器和 CPU 不做任何限制，可以乱序
- **Release 释放**，设定内存屏障(Memory barrier)，保证它之前的操作永远在它之前，但是它后面的操作可能被重排到它前面
- **Acquire 获取**, 设定内存屏障，保证在它之后的访问永远在它之后，但是它之前的操作却有可能被重排到它后面，往往和`Release`在不同线程中联合使用
- **AcqRel**, 是 *Acquire* 和 *Release* 的结合，同时拥有它们俩提供的保证。比如你要对一个 `atomic` 自增 1，同时希望该操作之前和之后的读取或写入操作不会被重新排序
- **SeqCst 顺序一致性**， `SeqCst`就像是`AcqRel`的加强版，它不管原子操作是属于读取还是写入的操作，只要某个线程有用到`SeqCst`的原子操作，线程中该`SeqCst`操作前的数据操作绝对不会被重新排在该`SeqCst`操作之后，且该`SeqCst`操作后的数据操作也绝对不会被重新排在`SeqCst`操作前。

这些规则由于是系统提供的，因此其它语言提供的相应规则也大同小异，大家如果不明白可以看看其它语言的相关解释。

#### 内存屏障的例子

下面我们以`Release`和`Acquire`为例，使用它们构筑出一对内存屏障，防止编译器和 CPU 将屏障前(Release)和屏障后(Acquire)中的数据操作重新排在屏障围成的范围之外:

```rust
use std::thread::{self, JoinHandle};
use std::sync::atomic::{Ordering, AtomicBool};

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn reset() {
    unsafe {
        DATA = 0;
    }
    READY.store(false, Ordering::Relaxed);
}

fn producer() -> JoinHandle<()> {
    thread::spawn(move || {
        unsafe {
            DATA = 100;                                 // A
        }
        READY.store(true, Ordering::Release);           // B: 内存屏障 ↑
    })
}

fn consumer() -> JoinHandle<()> {
    thread::spawn(move || {
        while !READY.load(Ordering::Acquire) {}         // C: 内存屏障 ↓

        assert_eq!(100, unsafe { DATA });               // D
    })
}


fn main() {
    loop {
        reset();

        let t_producer = producer();
        let t_consumer = consumer();

        t_producer.join().unwrap();
        t_consumer.join().unwrap();
    }
}
```

原则上，`Acquire`用于读取，而`Release`用于写入。但是由于有些原子操作同时拥有读取和写入的功能，此时就需要使用`AcqRel`来设置内存顺序了。在内存屏障中被写入的数据，都可以被其它线程读取到，不会有 CPU 缓存的问题。

**内存顺序的选择**

1. 不知道怎么选择时，优先使用`SeqCst`，虽然会稍微减慢速度，但是慢一点也比出现错误好
2. 多线程只计数`fetch_add`而不使用该值触发其他逻辑分支的简单使用场景，可以使用`Relaxed`  
   参考 [Which std::sync::atomic::Ordering to use?](https://stackoverflow.com/questions/30407121/which-stdsyncatomicordering-to-use)

use std::marker::PhantomPinned;
use std::pin::Pin;

// ==========================================
// 1. 普通类型：默认自动实现 `Unpin`
// ==========================================
#[derive(Default)]
struct UnpinTracker {
    val: i32,
}

// ==========================================
// 2. 固定类型：显式声明自己 `!Unpin` (没有实现 Unpin)
// ==========================================
struct PinnedTracker {
    val: i32,
    // PhantomPinned 是一个“零大小类型”(ZST)。
    // 它不包含任何数据，唯一的作用就是告诉编译器：“这个类型禁止被移动！”
    _pin: PhantomPinned,
}

impl PinnedTracker {
    fn new(val: i32) -> Self {
        Self {
            val,
            _pin: PhantomPinned,
        }
    }

    // 注意：对于 !Unpin 类型，参数必须是 self: Pin<&mut Self>
    fn print_addr(self: Pin<&mut Self>) {
        // 1. 安全地将 Pin<&mut Self> 降级为 Pin<&Self>
        let pinned_ref = self.as_ref();
        // 2. 从 Pin<&Self> 中安全地提取出 &Self
        let data_ref = pinned_ref.get_ref();
        // 3. 获取真实的内存地址
        let addr = data_ref as *const Self as usize;

        println!(
            "   [PinnedTracker] 地址: 0x{:x}, 值: {}",
            addr, data_ref.val
        );
    }
}

fn main() {
    println!("=== 实验 1: Unpin 类型 (可以被 Pin 包装，但依然可以自由移动) ===");
    let mut normal = UnpinTracker { val: 42 };
    let addr1 = &normal as *const _ as usize;
    println!("1. 初始栈地址: 0x{:x}", addr1);

    // 将可变引用包装进 Pin
    let pinned_normal = Pin::new(&mut normal);

    // 核心特性：因为 UnpinTracker 实现了 Unpin，
    // Pin 允许你通过 into_inner 安全地“解封”，重新获得 &mut T
    let normal_ref: &mut UnpinTracker = Pin::into_inner(pinned_normal);
    normal_ref.val = 100;
    let addr2 = normal_ref as *const _ as usize;
    println!("2. 从 Pin 中解封并修改后，地址: 0x{:x}", addr2);
    println!("   -> 结论: 对 Unpin 类型使用 Pin，就像给自行车上了个塑料玩具锁，一拔就开。\n");

    println!("=== 实验 2: !Unpin 类型 (被 Pin 彻底锁死，拒绝移动) ===");
    // 对于 !Unpin 类型，最安全的做法是分配到堆上并固定 (Box::pin)
    let mut boxed_pinned = Box::pin(PinnedTracker::new(99));

    // 我们可以安全地读取它 (as_mut 返回 Pin<&mut PinnedTracker>)
    boxed_pinned.as_mut().print_addr();

    // 灾难演示：尝试解封 !Unpin 类型 (请取消下一行的注释来观察编译器的愤怒)
    // let inner_mut: &mut PinnedTracker = Pin::into_inner(boxed_pinned);

    println!("   -> 结论: 如果取消上面那行的注释，编译器会直接报错：");
    println!("      `the trait bound PinnedTracker: Unpin is not satisfied`");
    println!("      Pin 拒绝交出 &mut T，从而在编译期彻底杜绝了移动它的可能。\n");

    println!("=== 实验 3: 为什么需要这个机制？(自引用结构的幽灵) ===");
    println!("假设有一个结构体：");
    println!("  struct SelfRef {{");
    println!("      data: String,");
    println!("      data_ptr: *const String, // 指向 data 的指针");
    println!("  }}");
    println!("如果这个结构体被移动，`data` 的内存地址会改变，但 `data_ptr` 里存的还是旧地址！");
    println!("这就产生了悬垂指针。");
    println!("Pin 的作用就是封印这个结构体：'只要你还想持有它的可变引用，你就休想移动它。'");
}

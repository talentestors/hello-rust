#[derive(Default)]
struct AddrTracker(Option<usize>);

impl AddrTracker {
    fn check_heap_addr(&mut self) {
        let current_addr = self as *mut Self as usize;
        match self.0 {
            None => {
                println!("   [堆数据] 首次记录地址: 0x{:x}", current_addr);
            }
            Some(prev_addr) => {
                if prev_addr != current_addr {
                    println!(
                        "   恐慌！堆数据从 0x{:x} 被移动到了 0x{:x}",
                        prev_addr, current_addr
                    );
                } else {
                    println!("   [堆数据] 地址未变，依然是: 0x{:x}", current_addr);
                }
            }
        }
        self.0 = Some(current_addr);
    }
}

// 辅助函数：同时打印栈上指针和堆上数据的地址
fn inspect_box(label: &str, tracker: &Box<AddrTracker>) {
    // 1. 获取 Box 变量本身在栈上的地址
    let stack_ptr_addr = tracker as *const _ as usize;
    // 2. 获取 Box 指向的堆数据的真实地址
    let heap_data_addr = tracker.as_ref() as *const AddrTracker as usize;

    println!("\n{}", label);
    println!("   [栈上指针变量] 地址: 0x{:x}", stack_ptr_addr);
    println!("   [堆上实际数据] 地址: 0x{:x}", heap_data_addr);
}

fn main() {
    println!("=== 1. 在 main 中初始化 ===");
    let mut tracker = Box::new(AddrTracker::default());

    inspect_box("调用 check 之前:", &tracker);
    tracker.check_heap_addr();

    println!("\n=== 2. 移动到 move_it 函数 ===");
    tracker = move_it(tracker);

    println!("\n=== 3. 回到 main 函数结尾 ===");
    inspect_box("函数返回后:", &tracker);
    tracker.check_heap_addr();
}

fn move_it(mut tracker: Box<AddrTracker>) -> Box<AddrTracker> {
    inspect_box("--- 在 move_it 内部 ---", &tracker);
    tracker.check_heap_addr();

    println!("\n  (准备将所有权返回给 main...)");
    tracker
}
/*
=== 1. 在 main 中初始化 ===

调用 check 之前:
   [栈上指针变量] 地址: 0xa9beeffbc8
   [堆上实际数据] 地址: 0x2394779c2c0
   [堆数据] 首次记录地址: 0x2394779c2c0

=== 2. 移动到 move_it 函数 ===

--- 在 move_it 内部 ---
   [栈上指针变量] 地址: 0xa9beeffb30
   [堆上实际数据] 地址: 0x2394779c2c0
   [堆数据] 地址未变，依然是: 0x2394779c2c0

  (准备将所有权返回给 main...)

=== 3. 回到 main 函数结尾 ===

函数返回后:
   [栈上指针变量] 地址: 0xa9beeffbc8
   [堆上实际数据] 地址: 0x2394779c2c0
   [堆数据] 地址未变，依然是: 0x2394779c2c0
*/

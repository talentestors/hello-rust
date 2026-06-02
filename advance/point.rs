#[derive(Default)]
struct AddrTracker(Option<usize>);

impl AddrTracker {
    // 检查自己的内存地址是否发生了变化
    fn check_for_move(&mut self) {
        let current_addr = self as *mut Self as usize;
        println!("\n当前地址: 0x{:x}", current_addr);
        match self.0 {
            None => (),
            Some(prev_addr) => {
                if prev_addr != current_addr {
                    println!(
                        "恐慌！我从地址 0x{:x} 被移动到了 0x{:x}",
                        prev_addr, current_addr
                    );
                }
            }
        }
        // 记录地址
        self.0 = Some(current_addr);
    }
}

fn main() {
    let mut tracker = AddrTracker::default();
    tracker.check_for_move();
    tracker = AddrTracker(Some(0));
    tracker.check_for_move();
    tracker.check_for_move();

    // let mut t = tracker;

    // t.check_for_move();
    // // 将 tracker 移动到一个新函数
    // tracker = move_it(t);

    // println!("--- 回到 main 函数 ---");
    // tracker.check_for_move();

    // let mut tracker2 = move_it(tracker);
    // tracker2.check_for_move();

    // // 修改原始slot
    // println!("\n ===== 修改原始slot =====");
    // tracker = AddrTracker(Some(0));
    // tracker.check_for_move();
    // tracker = move_it(tracker2);
    // tracker.check_for_move();
    // tracker.0 = Some(222);
    // tracker.check_for_move();
}

// fn move_it(mut tracker: AddrTracker) -> AddrTracker {
//     println!("--- 在 move_it 函数内部 ---");
//     tracker.check_for_move();
//     tracker
// }

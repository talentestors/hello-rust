use std::arch::asm;

fn main() {
    // three entries of four bytes each
    let mut name_buf = [0_u8; 12];
    // String is stored as ascii in ebx, edx, ecx in order
    // Because ebx is reserved, the asm needs to preserve the value of it.
    // So we push and pop it around the main asm.
    // (in 64 bit mode for 64 bit processors, 32 bit processors would use ebx)

    unsafe {
        asm!(
            "push rbx",
            "cpuid", // CPUID 返回的 ASCII 字符串在内存中的顺序规定为：先 ebx 的低字节到高字节，然后是 edx，最后是 ecx
            "mov [rdi], ebx",
            "mov [rdi + 4], edx",
            "mov [rdi + 8], ecx",
            "pop rbx",
            // We use a pointer to an array for storing the values to simplify
            // the Rust code at the cost of a couple more asm instructions
            // This is more explicit with how the asm works however, as opposed
            // to explicit register outputs such as `out("ecx") val`
            // The *pointer itself* is only an input even though it's written behind
            in("rdi") name_buf.as_mut_ptr(),
            // select cpuid 0, also specify eax as clobbered
            inout("eax") 0 => _,
            // cpuid clobbers these registers too
            out("ecx") _,
            out("edx") _,
        );
    }

    let name = core::str::from_utf8(&name_buf).unwrap();
    println!("CPU Manufacturer ID: {}", name);
}

// use std::arch::asm;

// Multiply x by 6 using shifts and adds
// let mut x: u64 = 4;
// unsafe {
//     asm!(
//         "mov {tmp}, {x}", // 复制x到tmp
//         "shl {tmp}, 1",  // tmp左移1位（乘2）
//         "shl {x}, 2",    // x左移2位（乘4）
//         "add {x}, {tmp}", // 相加并存入x x + tmp = 4*2+4*4 = 4*(2+4) = 4*6
//         x = inout(reg) x, // x 既是输入也是输出，使用通用寄存器（reg）
//         tmp = out(reg) _, // tmp 是只写输出；‘_’表示不需要把值赋给某个 Rust 变量
//     );
// }
// assert_eq!(x, 4 * 6);

// use std::arch::asm;

// let i: u64 = 3;
// let o: u64;
// unsafe {
//     asm!(
//         "mov {0}, {1}",
//         "add {0}, 5",
//         out(reg) o, // 0
//         in(reg) i, // 1
//     );
// }
// assert_eq!(o, 8);

// use std::arch::asm;

// let x: u64;
// unsafe {
//     asm!("mov {}, 5", out(reg) x);
// }
// assert_eq!(x, 5);

// use std::arch::asm;

// unsafe {
//     asm!("nop");
// }

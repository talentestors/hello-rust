# 包 Crate

## 典型的 Package 结构

上面创建的 Package 中仅包含 `src/main.rs` 文件，意味着它仅包含一个二进制同名包 my-project。如果一个 Package 同时拥有 `src/main.rs` 和 `src/lib.rs`，那就意味着它包含两个包：库包和二进制包，这两个包名也都是 my-project —— 都与 Package 同名。

一个真实项目中典型的 Package，会包含多个二进制包，这些包文件被放在 `src/bin` 目录下，每一个文件都是独立的二进制包，同时也会包含一个库包，该包只能存在一个 `src/lib.rs`：

```
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests
│   └── some_integration_tests.rs
├── benches
│   └── simple_bench.rs
└── examples
    └── simple_example.rs
```

唯一库包：`src/lib.rs`
默认二进制包：`src/main.rs`，编译后生成的可执行文件与 Package 同名
其余二进制包：`src/bin/main1.rs` 和 `src/bin/main2.rs`，它们会分别生成一个文件同名的二进制可执行文件
集成测试文件：`tests` 目录下
基准性能测试 benchmark 文件：`benches` 目录下
项目示例：`examples` 目录下
这种目录结构基本上是 Rust 的标准目录结构，在 GitHub 的大多数项目上，你都将看到它的身影。

理解了包的概念，我们再来看看构成包的基本单元：模块。

# 结构体和枚举的可见性

为何要把结构体和枚举的可见性单独拎出来讲呢？因为这两个家伙的成员字段拥有完全不同的可见性：

- 将结构体设置为 `pub`，但它的所有字段依然是私有的将枚举设置为 `pub`，它的所有字段也将对外可见
- 原因在于，枚举和结构体的使用方式不一样。如果枚举的成员对外不可见，那该枚举将一点用都没有，因此枚举成员的可见性自动跟枚举可见性保持一致，这样可以简化用户的使用。

而结构体的应用场景比较复杂，其中的字段也往往部分在 A 处被使用，部分在 B 处被使用，因此无法确定成员的可见性，那索性就设置为全部不可见，将选择权交给程序员。

# Criterion.rs 基准测试笔记

## 1. 简介

Criterion.rs 是 Rust 生态中最成熟的**稳定版兼容**基准测试框架，提供：

- 统计可靠的测量（置信区间、离群值检测）
- 自动比较性能变化（回归/改进）
- 生成 HTML 报告和图表
- 支持多平台（无需 nightly）

## 2. 项目配置

### 2.1 在 `Cargo.toml` 中添加依赖

```toml
[dev-dependencies]
criterion = "0.8.2"      # 最新版本请查 crates.io
rand = "0.10.1"          # 可选，用于生成测试数据

[[bench]]
name = "sum_bench"       # benchmark 名称
harness = false          # 禁用默认测试框架，由 criterion 接管
```

### 2.2 优化选项（推荐）

```toml
[profile.bench]
codegen-units = 1        # 提升优化质量，但编译变慢
lto = true               # 链接时优化
```

> 这些选项会显著增加编译时间，但对性能测量更准确。

## 3. 编写基准测试

在 `benches/sum_bench.rs` 中：

```rust
use criterion::{criterion_group, criterion_main, Criterion};
use rand::{Rng, thread_rng};

// 待测函数
fn sum_for(x: &[f64]) -> f64 {
    let mut result = 0.0;
    for i in 0..x.len() {
        result += x[i];
    }
    result
}

fn sum_iter(x: &[f64]) -> f64 {
    x.iter().sum()
}

// 基准测试函数（名称任意）
fn bench_sum(c: &mut Criterion) {
    let len = 1024 * 1024;
    let samples: Vec<f64> = (0..len)
        .map(|_| thread_rng().r#gen())   // 注意保留关键字 gen 需转义
        .collect();

    // 注册两个测试项
    c.bench_function("for_loop", |b| b.iter(|| sum_for(&samples)));
    c.bench_function("iterator", |b| b.iter(|| sum_iter(&samples)));
}

// 生成测试组
criterion_group!(benches, bench_sum);
criterion_main!(benches);
```

### 3.1 要点说明

- `criterion_group!` 将基准函数组合成一个组。
- `criterion_main!` 生成 `main` 函数。
- `b.iter(|| ...)` 中的代码是**被测量的核心**，会反复执行。
- 测试数据的生成应放在闭包**外部**，避免计入测量时间。
- 使用 `r#gen` 绕过 `gen` 关键字（Rust 2024+）。

## 4. 运行基准测试

```bash
cargo bench                    # 运行所有基准
cargo bench -- for_loop        # 只运行名称包含 "for_loop" 的测试
cargo bench -- --help          # 查看 Criterion 自带命令行选项
```

### 常用选项

- `--save-baseline <name>` – 保存当前结果为基线
- `--baseline <name>` – 与指定基线比较
- `--plotting-backend gnuplot` – 使用 gnuplot 生成图表（需安装 gnuplot）

## 5. 理解输出结果

典型输出（节选）：

```
for_loop                time:   [565.16 µs 593.98 µs 628.55 µs]
                        change: [+1.8530% +5.7080% +10.117%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
```

### 5.1 时间区间 `[低 中位 高]`

- **低 / 高**：95% 置信区间边界（默认），表示真实性能有 95% 的概率落在此区间。
- **中位**：点估计值（通常是中位数），代表典型性能。

> 如果区间宽度很大（例如 ±20%），说明测试环境不稳定。

### 5.2 变化百分比 `[低 中位 高]`

- 与**上一次运行**（或指定基线）比较。
- 负数 = 性能提升（耗时变短）；正数 = 性能退化。
- 同样带有置信区间。

### 5.3 p 值

- `p < 0.05` 表示变化具有统计显著性（不是随机噪声）。
- `p = 0.00` 代表实际值极小，可视为显著。

### 5.4 离群值

- `low mild / high mild` – 轻度离群（1.5~3 倍 IQR）
- `low severe / high severe` – 严重离群（>3 倍 IQR）
- 离群率 > 5% 说明测试环境噪声大，应排查。

### 5.5 结论

- `Performance has improved` – 统计显著且耗时降低
- `Performance has regressed` – 统计显著且耗时增加
- 若无显著变化则不输出

## 6. 高级功能

### 6.1 参数化测试

使用 `bench_function_with_input` 或 `BenchmarkId`：

```rust
use criterion::{BenchmarkId, Criterion};

fn bench_sizes(c: &mut Criterion) {
    let sizes = [1000, 10000, 100000];
    let mut group = c.benchmark_group("sum_by_size");
    for &size in &sizes {
        let data = vec![0.0; size];
        group.bench_with_input(BenchmarkId::new("for_loop", size), &data, |b, d| {
            b.iter(|| sum_for(d))
        });
    }
    group.finish();
}
```

### 6.2 吞吐量测量

```rust
c.bench_function("copy", |b| {
    b.iter(|| heavy_computation());
    b.bytes = 1024 * 1024;   // 每次迭代处理的字节数
});
```

输出中会出现 `Throughput: 1.2 GiB/s`。

### 6.3 自定义配置

```rust
use criterion::{Criterion, measurement::WallTime};

fn config() -> Criterion<WallTime> {
    Criterion::default()
        .sample_size(500)            // 采样次数（默认100）
        .warm_up_time(std::time::Duration::from_secs(3))
        .measurement_time(std::time::Duration::from_secs(5))
        .significance_level(0.01)    // p 值阈值（默认0.05）
}

criterion_group! {
    name = benches;
    config = config();
    targets = bench_sum
}
```

### 6.4 比较两个函数

```rust
c.bench_function("for", |b| b.iter(|| sum_for(&data)));
c.bench_function("iter", |b| b.iter(|| sum_iter(&data)));
```

Criterion 会自动比较并给出 `change` 字段。

## 7. 结果报告

运行完成后，会在 `target/criterion` 目录生成报告：

- `report/index.html` – 总览页面
- 每个基准测试对应一个子目录，内有箱线图、密度图、历史趋势图

用浏览器打开即可直观分析。

## 8. 最佳实践总结

| 实践                                     | 原因                                          |
| ---------------------------------------- | --------------------------------------------- |
| 将测试数据生成移出 `b.iter`              | 避免计入测量时间                              |
| 启用 `codegen-units = 1` 和 `lto = true` | 获得真实发布优化                              |
| 关闭后台 CPU/内存密集型程序              | 减少系统噪声                                  |
| 使用 `black_box` 防止结果优化掉          | 对未被使用的返回值可用 `std::hint::black_box` |
| 至少运行两次基准后再依赖 `change`        | 第一次无历史比较基线                          |
| 关注中位值和置信区间宽度                 | 中位代表典型性能，宽度代表稳定性              |
| 对于微小变化（<2%）且 p<0.05，仍可忽略   | 实际应用无感知                                |

### 8.1 使用 `black_box` 示例

```rust
use std::hint::black_box;

b.iter(|| black_box(sum_for(black_box(&samples))));
```

防止编译器提前内联计算出结果而跳过实际计算。

## 9. 常见问题

**Q: 为什么 `gen` 报错？**  
A: Rust 2024 起 `gen` 成为保留关键字，使用 `r#gen()` 或升级 `rand` 到 0.9+ 使用 `random()`。

**Q: 提示 `Gnuplot not found` 有影响吗？**  
A: 无影响，Criterion 会自动回退到纯 Rust 绘图后端 `plotters`。如需更美观图表可安装 gnuplot。

**Q: 如何只保存一次基线，多次与它比较？**  
A: 第一次运行：`cargo bench -- --save-baseline main`  
后续运行：`cargo bench -- --baseline main`

**Q: 基准测试编译很慢？**  
A: 正常现象，因为 `profile.bench` 优化级别高且启用了 LTO。可在开发时暂时注释优化。

---

## 10. 参考资料

- [Criterion.rs 官方文档](https://bheisler.github.io/criterion.rs/book/)
- [GitHub 仓库](https://github.com/bheisler/criterion.rs)
- [Rust 性能书](https://nnethercote.github.io/perf-book/benchmarking.html)

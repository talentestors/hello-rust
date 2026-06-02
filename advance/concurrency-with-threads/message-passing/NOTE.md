# mpmc 更好的性能

如果你需要 mpmc(多发送者，多接收者)或者需要更高的性能，可以考虑第三方库:

- [crossbeam-channel](https://github.com/crossbeam-rs/crossbeam/tree/master/crossbeam-channel), 老牌强库，功能较全，性能较强，之前是独立的库，但是后面合并到了crossbeam主仓库中
- [flume](https://github.com/zesterer/flume), 官方给出的性能数据某些场景要比 crossbeam 更好些

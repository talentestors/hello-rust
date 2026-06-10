use tracing::{Level, span};
fn main() {
    let span = span!(Level::TRACE, "my_span");

    // `enter` 返回一个 RAII ，当其被 drop 时，将自动结束该 span
    let enter = span.enter();
    // 这里开始进入 `my_span` 的上下文
    // 下面执行一些任务，并记录一些信息到 `my_span` 中
    // ...
} // 这里 enter 将被 drop，`my_span` 也随之结束

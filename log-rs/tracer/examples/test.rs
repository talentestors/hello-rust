use tracing::{Level, event, span};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let s = span!(Level::TRACE, "my span");
    // 没进入 span，因此输出日志将不会带上 span 的信息
    event!(target: "app_events", Level::INFO, "something has happened 1!");

    // 进入 span ( 开始 )
    let _enter = s.enter();
    // 没有设置 target 和 parent
    // 这里的对象位置分别是当前的 span 名和模块名
    event!(Level::INFO, "something has happened 2!");
    // 设置了 target
    // 这里的对象位置分别是当前的 span 名和 target
    event!(target: "app_events",Level::INFO, "something has happened 3!");

    let span = span!(Level::TRACE, "my span 1");
    // 这里就更为复杂一些，留给大家作为思考题
    event!(parent: &span, Level::INFO, "something has happened 4!");
}

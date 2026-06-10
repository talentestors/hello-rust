use tracing::{info, instrument};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[instrument]
fn foo(ans: i32) {
    info!("in foo");
}

fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();
    foo(42);
}

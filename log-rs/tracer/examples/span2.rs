use tracing::{Level, debug, info, span};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let scope = span!(Level::DEBUG, "foo");
    let _enter = scope.enter();
    info!("Hello in foo scope");
    debug!("before entering bar scope");
    {
        let scope = span!(Level::DEBUG, "bar", ans = 42);
        let _enter = scope.enter();
        debug!("enter bar scope");
        info!("In bar scope");
        debug!("end bar scope");
    }
    debug!("end bar scope");
}

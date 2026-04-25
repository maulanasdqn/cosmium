use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;

pub fn init() {
    let filter = EnvFilter::try_from_env("COSMIUM_LOG")
        .or_else(|_| EnvFilter::try_new("info,cosmium=debug,engine=debug"))
        .expect("valid log filter");

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .compact()
        .init();
}

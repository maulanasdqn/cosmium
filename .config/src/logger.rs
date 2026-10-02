use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;

pub fn init() {
    let filter = EnvFilter::try_from_env("COSMIUM_LOG").unwrap_or_else(|_| {
        EnvFilter::new("info,cosmium=debug,engine=debug,chromiumoxide::handler=error")
    });

    fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .with_target(false)
        .compact()
        .init();
}

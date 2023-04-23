use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub fn init() {
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(false)
                .with_file(true)
                .with_line_number(true)
                .log_internal_errors(true),
        )
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
}

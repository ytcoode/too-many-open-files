use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init() {
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_target(false)
                .with_file(true)
                .with_line_number(true)
                .log_internal_errors(true),
        )
        .with(EnvFilter::from_default_env())
        .init();
}

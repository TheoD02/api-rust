// src/config/logging.rs
// Equivalent de: config/packages/monolog.yaml

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize logging
/// Equivalent de: monolog configuration en Symfony
pub fn init_logging() {
    // RUST_LOG env var (comme MONOLOG_LEVEL)
    // Examples:
    //   RUST_LOG=debug
    //   RUST_LOG=rust_api=debug,tower_http=info
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        // Default: info for app, warn for dependencies
        EnvFilter::new("rust_api=info,tower_http=info,sea_orm=warn")
    });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_level(true)
                .with_thread_ids(false)
                .with_file(false)
                .with_line_number(false),
        )
        .init();
}

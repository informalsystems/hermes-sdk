/**
   Install the [`tracing_subscriber`] logger handlers so that logs will
   be displayed during test.
*/
pub fn install_logger(with_color: bool) {
    use tracing::level_filters::LevelFilter;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;
    use tracing_subscriber::EnvFilter;

    // Use log level INFO by default if RUST_LOG is not set.
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let layer = tracing_subscriber::fmt::layer().with_ansi(with_color);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(layer)
        .init();
}

/// Check if both stdout and stderr are proper terminal (tty),
/// so that we know whether or not to enable colored output,
/// using ANSI escape codes. If either is not, eg. because
/// stdout is redirected to a file, we don't enable colored output.
pub fn enable_ansi() -> bool {
    use std::io::IsTerminal;
    std::io::stdout().is_terminal() && std::io::stderr().is_terminal()
}

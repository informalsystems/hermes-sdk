use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::EnvFilter;

/**
   Install the [`tracing_subscriber`] logger handlers so that logs will
   be displayed during test.
*/
pub fn install_logger(with_color: bool, with_json: bool) {
    use tracing_subscriber::fmt;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let target_filter = Targets::new().with_target("hermes", Level::TRACE);

    let subscriber = tracing_subscriber::registry()
        .with(target_filter)
        .with(env_filter);

    if with_json {
        let fmt_layer = fmt::layer().with_target(false).json();

        let _ = subscriber.with(fmt_layer).try_init();
    } else {
        let fmt_layer = fmt::layer()
            .with_ansi(with_color)
            .with_target(false)
            .compact();

        let _ = subscriber.with(fmt_layer).try_init();
    };
}

/// Check if both stdout and stderr are proper terminal (tty),
/// so that we know whether or not to enable colored output,
/// using ANSI escape codes. If either is not, eg. because
/// stdout is redirected to a file, we don't enable colored output.
pub fn enable_ansi() -> bool {
    use std::io::IsTerminal;
    std::io::stdout().is_terminal() && std::io::stderr().is_terminal()
}

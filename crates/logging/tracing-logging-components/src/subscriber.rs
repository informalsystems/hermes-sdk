use tracing::{Level, Subscriber};
use tracing_subscriber::filter::{LevelFilter, Targets};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

pub fn build_tracing_subscriber() -> impl Subscriber {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let target_filter = Targets::new().with_target("hermes", Level::TRACE);

    tracing_subscriber::registry()
        .with(target_filter)
        .with(env_filter)
        .with(fmt::layer().with_target(false))
}

pub fn init_tracing_subscriber() {
    let subscriber = build_tracing_subscriber();

    // Avoid crashing if already initialised
    let _ = subscriber.try_init();
}

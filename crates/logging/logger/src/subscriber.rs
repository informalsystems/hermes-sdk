use tracing::Level;
use tracing_subscriber::filter::{LevelFilter, Targets};
use tracing_subscriber::fmt::Layer;
use tracing_subscriber::layer::{Layered, SubscriberExt};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter, Registry};

pub fn build_tracing_subscriber(
) -> Layered<EnvFilter, Layered<Targets, Layered<Layer<Registry>, Registry>>> {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    let target_filter = Targets::new().with_target("hermes", Level::TRACE);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(target_filter)
        .with(env_filter)
}

pub fn init_tracing_subscriber() {
    let subscriber = build_tracing_subscriber();

    // Avoid crashing if already initialised
    let _ = subscriber.try_init();
}

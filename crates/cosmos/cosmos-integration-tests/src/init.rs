use alloc::sync::Arc;

use hermes_runtime::types::runtime::HermesRuntime;
use tokio::runtime::Builder;
use tracing::info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

pub fn init_test_runtime() -> HermesRuntime {
    let _ = stable_eyre::install();

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(env_filter)
        .init();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build().unwrap());

    let runtime = HermesRuntime::new(tokio_runtime.clone());

    info!("initialized Hermes test runtime");

    runtime
}

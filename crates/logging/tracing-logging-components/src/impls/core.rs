use cgp::core::Async;
use hermes_logging_components::traits::logger::Logger;
use hermes_logging_components::types::level::{
    LevelDebug, LevelError, LevelInfo, LevelTrace, LevelWarn,
};
use tracing::{debug, error, info, trace, warn};

use crate::contexts::logger::TracingLogger;

impl<Logging> Logger<Logging, ()> for TracingLogger
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &()) {
        info!("{message}");
    }
}

impl<Logging> Logger<Logging, LevelTrace> for TracingLogger
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &LevelTrace) {
        trace!("{message}");
    }
}

impl<Logging> Logger<Logging, LevelDebug> for TracingLogger
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &LevelDebug) {
        debug!("{message}");
    }
}

impl<Logging> Logger<Logging, LevelInfo> for TracingLogger
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &LevelInfo) {
        info!("{message}");
    }
}

impl<Logging> Logger<Logging, LevelWarn> for TracingLogger
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &LevelWarn) {
        warn!("{message}");
    }
}

impl<Logging> Logger<Logging, LevelError> for TracingLogger
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &LevelError) {
        error!("{message}");
    }
}

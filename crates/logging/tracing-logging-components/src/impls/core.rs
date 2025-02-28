use cgp::prelude::*;
use hermes_logging_components::traits::logger::{Logger, LoggerComponent};
use hermes_logging_components::types::level::{
    LevelDebug, LevelError, LevelInfo, LevelTrace, LevelWarn,
};
use hermes_relayer_components::error::impls::retry::LogPerformRetry;
use tracing::{debug, error, info, trace, warn};

use crate::contexts::logger::TracingLogger;

#[cgp_provider(LoggerComponent)]
impl<Logging> Logger<Logging, ()> for TracingLogger
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &()) {
        info!("{message}");
    }
}

#[cgp_provider(LoggerComponent)]
impl<Logging> Logger<Logging, LevelTrace> for TracingLogger
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &LevelTrace) {
        trace!("{message}");
    }
}

#[cgp_provider(LoggerComponent)]
impl<Logging> Logger<Logging, LevelDebug> for TracingLogger
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &LevelDebug) {
        debug!("{message}");
    }
}

#[cgp_provider(LoggerComponent)]
impl<Logging> Logger<Logging, LevelInfo> for TracingLogger
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &LevelInfo) {
        info!("{message}");
    }
}

#[cgp_provider(LoggerComponent)]
impl<Logging> Logger<Logging, LevelWarn> for TracingLogger
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &LevelWarn) {
        warn!("{message}");
    }
}

#[cgp_provider(LoggerComponent)]
impl<Logging> Logger<Logging, LevelError> for TracingLogger
where
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, _details: &LevelError) {
        error!("{message}");
    }
}

#[cgp_provider(LoggerComponent)]
impl<'a, Logging, Context> Logger<Logging, LogPerformRetry<'a, Context>> for TracingLogger
where
    Context: HasAsyncErrorType,
    Logging: Async,
{
    async fn log(_logging: &Logging, message: &str, details: &LogPerformRetry<'a, Context>) {
        info!(
            task_name = %details.task_name,
            error = ?details.error,
            attempts = %details.attempts,
            max_retries = %details.max_retries,
            retry_interval = ?details.retry_interval,
            "{message}",
        )
    }
}

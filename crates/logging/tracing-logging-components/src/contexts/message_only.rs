use core::marker::PhantomData;

use hermes_logging_components::traits::{Logger, LoggerComponent};
use hermes_logging_components::types::{LevelDebug, LevelError, LevelInfo, LevelTrace, LevelWarn};
use hermes_prelude::*;
use tracing::{debug, error, info, trace, warn};

pub struct LogMessageOnly<Level>(pub PhantomData<Level>);

#[cgp_provider(LoggerComponent)]
impl<Logging, Details> Logger<Logging, Details> for LogMessageOnly<LevelError>
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, message: &str, _details: &Details) {
        error!("{message}");
    }
}

#[cgp_provider(LoggerComponent)]
impl<Logging, Details> Logger<Logging, Details> for LogMessageOnly<LevelWarn>
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, message: &str, _details: &Details) {
        warn!("{message}");
    }
}

#[cgp_provider(LoggerComponent)]
impl<Logging, Details> Logger<Logging, Details> for LogMessageOnly<LevelInfo>
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, message: &str, _details: &Details) {
        info!("{message}");
    }
}

#[cgp_provider(LoggerComponent)]
impl<Logging, Details> Logger<Logging, Details> for LogMessageOnly<LevelDebug>
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, message: &str, _details: &Details) {
        debug!("{message}");
    }
}

#[cgp_provider(LoggerComponent)]
impl<Logging, Details> Logger<Logging, Details> for LogMessageOnly<LevelTrace>
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, message: &str, _details: &Details) {
        trace!("{message}");
    }
}

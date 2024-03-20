use core::marker::PhantomData;

use cgp_core::Async;
use hermes_relayer_components::log::traits::logger::Logger;
use hermes_relayer_components::log::types::level::{
    LevelDebug, LevelError, LevelInfo, LevelTrace, LevelWarn,
};
use log::{debug, error, info, trace, warn};
pub struct LogMessageOnly<Level>(pub PhantomData<Level>);

impl<Logging, Details> Logger<Logging, Details> for LogMessageOnly<LevelError>
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, message: &str, _details: &Details) {
        error!("{message}");
    }
}

impl<Logging, Details> Logger<Logging, Details> for LogMessageOnly<LevelWarn>
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, message: &str, _details: &Details) {
        warn!("{message}");
    }
}

impl<Logging, Details> Logger<Logging, Details> for LogMessageOnly<LevelInfo>
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, message: &str, _details: &Details) {
        info!("{message}");
    }
}

impl<Logging, Details> Logger<Logging, Details> for LogMessageOnly<LevelDebug>
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, message: &str, _details: &Details) {
        debug!("{message}");
    }
}

impl<Logging, Details> Logger<Logging, Details> for LogMessageOnly<LevelTrace>
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, message: &str, _details: &Details) {
        trace!("{message}");
    }
}

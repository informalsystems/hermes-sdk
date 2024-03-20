use core::marker::PhantomData;

use cgp_core::Async;
use hermes_relayer_components::log::traits::logger::Logger;
use hermes_relayer_components::log::types::level::{
    LevelDebug, LevelError, LevelInfo, LevelTrace, LevelWarn,
};
use log::{debug, error, info, trace, warn};
use serde::Serialize;

pub struct LogSerialize<Level>(pub PhantomData<Level>);

impl<Logging, Details> Logger<Logging, Details> for LogSerialize<LevelError>
where
    Logging: Serialize + Async,
    Details: Serialize + Send + Sync,
{
    async fn log(logging: &Logging, message: &str, details: &Details) {
        error!(context:serde = logging, details:serde; "{message}");
    }
}

impl<Logging, Details> Logger<Logging, Details> for LogSerialize<LevelWarn>
where
    Logging: Serialize + Async,
    Details: Serialize + Send + Sync,
{
    async fn log(logging: &Logging, message: &str, details: &Details) {
        warn!(context:serde = logging, details:serde; "{message}");
    }
}

impl<Logging, Details> Logger<Logging, Details> for LogSerialize<LevelInfo>
where
    Logging: Serialize + Async,
    Details: Serialize + Send + Sync,
{
    async fn log(logging: &Logging, message: &str, details: &Details) {
        info!(context:serde = logging, details:serde; "{message}");
    }
}

impl<Logging, Details> Logger<Logging, Details> for LogSerialize<LevelDebug>
where
    Logging: Serialize + Async,
    Details: Serialize + Send + Sync,
{
    async fn log(logging: &Logging, message: &str, details: &Details) {
        debug!(context:serde = logging, details:serde; "{message}");
    }
}

impl<Logging, Details> Logger<Logging, Details> for LogSerialize<LevelTrace>
where
    Logging: Serialize + Async,
    Details: Serialize + Send + Sync,
{
    async fn log(logging: &Logging, message: &str, details: &Details) {
        trace!(context:serde = logging, details:serde; "{message}");
    }
}

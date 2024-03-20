use core::marker::PhantomData;

use cgp_core::Async;
use hermes_relayer_components::log::traits::logger::Logger;
use hermes_relayer_components::log::types::level::{
    LevelDebug, LevelError, LevelInfo, LevelTrace, LevelWarn,
};
use log::{debug, error, info, trace, warn};
use serde::Serialize;

pub struct LogSerialize<Level>(pub PhantomData<Level>);

impl<Context, Details> Logger<Context, Details> for LogSerialize<LevelError>
where
    Context: Async,
    Details: Serialize + Send + Sync,
{
    async fn log(_context: &Context, message: &str, details: Details) {
        error!(details:serde; "{message}");
    }
}

impl<Context, Details> Logger<Context, Details> for LogSerialize<LevelWarn>
where
    Context: Async,
    Details: Serialize + Send + Sync,
{
    async fn log(_context: &Context, message: &str, details: Details) {
        warn!(details:serde; "{message}");
    }
}

impl<Context, Details> Logger<Context, Details> for LogSerialize<LevelInfo>
where
    Context: Async,
    Details: Serialize + Send + Sync,
{
    async fn log(_context: &Context, message: &str, details: Details) {
        info!(details:serde; "{message}");
    }
}

impl<Context, Details> Logger<Context, Details> for LogSerialize<LevelDebug>
where
    Context: Async,
    Details: Serialize + Send + Sync,
{
    async fn log(_context: &Context, message: &str, details: Details) {
        debug!(details:serde; "{message}");
    }
}

impl<Context, Details> Logger<Context, Details> for LogSerialize<LevelTrace>
where
    Context: Async,
    Details: Serialize + Send + Sync,
{
    async fn log(_context: &Context, message: &str, details: Details) {
        trace!(details:serde; "{message}");
    }
}

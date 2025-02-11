use cgp::prelude::*;

use crate::traits::logger::{Logger, LoggerComponent};

pub struct IgnoreLog;

#[cgp_provider(LoggerComponent)]
impl<Logging, Details> Logger<Logging, Details> for IgnoreLog
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, _message: &str, _details: &Details) {}
}

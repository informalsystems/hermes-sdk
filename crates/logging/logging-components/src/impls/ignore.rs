use cgp::prelude::*;

use crate::traits::{Logger, LoggerComponent};

#[cgp_new_provider(LoggerComponent)]
impl<Logging, Details> Logger<Logging, Details> for IgnoreLog
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, _message: &str, _details: &Details) {}
}

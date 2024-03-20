use cgp_core::Async;

use crate::log::traits::logger::Logger;

pub struct IgnoreLog;

impl<Logging, Details> Logger<Logging, Details> for IgnoreLog
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, _message: &str, _details: Details) {}
}

use cgp::core::Async;

use crate::traits::logger::Logger;

pub struct IgnoreLog;

impl<Logging, Details> Logger<Logging, Details> for IgnoreLog
where
    Logging: Async,
    Details: Send + Sync,
{
    async fn log(_logging: &Logging, _message: &str, _details: &Details) {}
}

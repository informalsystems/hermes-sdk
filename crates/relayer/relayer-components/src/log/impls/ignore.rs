use cgp_core::Async;

use crate::log::traits::logger::Logger;

pub struct IgnoreLog;

impl<Context, Details> Logger<Context, Details> for IgnoreLog
where
    Context: Async,
    Details: Send + Sync,
{
    async fn log(_context: &Context, _message: &str, _details: Details) {}
}

use cgp_core::prelude::*;

#[derive_component(LoggerComponent, Logger<Context>)]
#[async_trait]
pub trait CanLog<Details>: Async
where
    Details: Send + Sync,
{
    async fn log(&self, message: &str, details: &Details);
}

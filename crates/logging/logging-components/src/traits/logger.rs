use cgp::prelude::*;

#[cgp_component {
  provider: Logger,
  context: Logging,
}]
#[async_trait]
pub trait CanLog<Details>: Async
where
    Details: Send + Sync,
{
    async fn log(&self, message: &str, details: &Details);
}

#[async_trait]
pub trait CanLogMessage: Async {
    async fn log_message(&self, message: &str);
}

impl<Logging> CanLogMessage for Logging
where
    Logging: CanLog<()>,
{
    async fn log_message(&self, message: &str) {
        self.log(message, &()).await
    }
}

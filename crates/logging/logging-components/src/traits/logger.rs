use cgp::core::component::UseDelegate;
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

#[cgp_provider(LoggerComponent)]
impl<Logging, Components, Delegate, Details> Logger<Logging, Details> for UseDelegate<Components>
where
    Logging: Async,
    Details: Send + Sync,
    Components: DelegateComponent<Details, Delegate = Delegate>,
    Delegate: Logger<Logging, Details>,
{
    async fn log(logging: &Logging, message: &str, details: &Details) {
        Delegate::log(logging, message, details).await
    }
}

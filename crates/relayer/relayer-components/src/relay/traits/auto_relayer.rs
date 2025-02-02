use cgp::prelude::*;

#[cgp_component {
  provider: AutoRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanAutoRelay<Target: Async>: Async + HasAsyncErrorType {
    async fn auto_relay(&self, target: Target) -> Result<(), Self::Error>;
}

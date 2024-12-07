use cgp::prelude::*;

#[cgp_component {
  name: AutoRelayerComponent,
  provider: AutoRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanAutoRelay<Target: Async>: Async + HasErrorType {
    async fn auto_relay(&self, target: Target) -> Result<(), Self::Error>;
}

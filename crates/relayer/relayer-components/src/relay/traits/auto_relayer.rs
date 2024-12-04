use cgp::prelude::*;

#[derive_component(AutoRelayerComponent, AutoRelayer<Relay>)]
#[async_trait]
pub trait CanAutoRelay<Target: Async>: Async + HasErrorType {
    async fn auto_relay(&self, target: Target) -> Result<(), Self::Error>;
}

use cgp::prelude::*;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_chain_components::types::aliases::HeightOf;

use crate::relay::traits::target::{HasTargetChainTypes, RelayTarget};

#[cgp_component {
    provider: AutoRelayer,
    context: Relay,
}]
#[async_trait]
pub trait CanAutoRelay<Target: Async>: HasAsyncErrorType {
    async fn auto_relay(&self, target: Target) -> Result<(), Self::Error>;
}

#[cgp_component {
    provider: AutoRelayerWithHeights,
    context: Relay,
}]
#[async_trait]
pub trait CanAutoRelayWithHeights<Target>:
    HasTargetChainTypes<Target, TargetChain: HasHeightType> + HasAsyncErrorType
where
    Target: RelayTarget,
{
    async fn auto_relay_with_heights(
        &self,
        target: Target,
        start_height: &HeightOf<Self::TargetChain>,
        end_height: Option<&HeightOf<Self::TargetChain>>,
    ) -> Result<(), Self::Error>;
}

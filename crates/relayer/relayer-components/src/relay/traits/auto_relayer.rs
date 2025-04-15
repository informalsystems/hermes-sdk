use cgp::prelude::*;
use hermes_chain_components::traits::HasHeightType;
use hermes_chain_components::types::aliases::HeightOf;

use crate::relay::traits::{HasTargetChainTypes, RelayTarget};

#[cgp_component {
    name: TargetAutoRelayerComponent,
    provider: TargetAutoRelayer,
    context: Relay,
}]
#[async_trait]
pub trait CanAutoRelayTarget<Target: Async>: HasAsyncErrorType {
    async fn auto_relay(&self, target: Target) -> Result<(), Self::Error>;
}

#[cgp_component {
    name: AutoRelayerWithHeightsComponent,
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

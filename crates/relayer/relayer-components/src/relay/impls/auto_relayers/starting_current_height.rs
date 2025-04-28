use cgp::core::error::ErrorOf;
use hermes_chain_components::traits::CanQueryChainHeight;
use hermes_prelude::*;

use crate::relay::traits::{
    CanAutoRelayWithHeights, HasTargetChains, RelayTarget, TargetAutoRelayer,
    TargetAutoRelayerComponent,
};

pub struct AutoRelayStartingCurrentHeight;

#[cgp_provider(TargetAutoRelayerComponent)]
impl<Relay, Target> TargetAutoRelayer<Relay, Target> for AutoRelayStartingCurrentHeight
where
    Relay: HasTargetChains<Target>
        + CanAutoRelayWithHeights<Target>
        + CanRaiseAsyncError<ErrorOf<Relay::TargetChain>>,
    Target: RelayTarget,
    Relay::TargetChain: CanQueryChainHeight,
{
    async fn auto_relay(relay: &Relay, target: Target) -> Result<(), Relay::Error> {
        let start_height = relay
            .target_chain()
            .query_chain_height()
            .await
            .map_err(Relay::raise_error)?;

        relay
            .auto_relay_with_heights(target, &start_height, None)
            .await
    }
}

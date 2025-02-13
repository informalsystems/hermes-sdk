use cgp::core::error::ErrorOf;
use cgp::prelude::*;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;

use crate::components::default::relay::AutoRelayerComponent;
use crate::relay::traits::auto_relayer::{AutoRelayer, CanAutoRelayWithHeights};
use crate::relay::traits::target::{HasTargetChains, RelayTarget};

pub struct AutoRelayStartingCurrentHeight;

#[cgp_provider(AutoRelayerComponent)]
impl<Relay, Target> AutoRelayer<Relay, Target> for AutoRelayStartingCurrentHeight
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

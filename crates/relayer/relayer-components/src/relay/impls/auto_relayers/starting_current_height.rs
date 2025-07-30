use core::time::Duration;

use cgp::core::error::ErrorOf;
use hermes_chain_components::traits::CanQueryChainHeight;
use hermes_prelude::*;

use crate::relay::traits::{
    CanAutoRelayWithHeights, CanRefreshClient, HasTargetChains, RelayTarget, TargetAutoRelayer,
    TargetAutoRelayerComponent,
};

pub struct AutoRelayStartingCurrentHeight;

#[cgp_provider(TargetAutoRelayerComponent)]
impl<Relay, Target> TargetAutoRelayer<Relay, Target> for AutoRelayStartingCurrentHeight
where
    Relay: HasTargetChains<Target>
        + CanAutoRelayWithHeights<Target>
        + CanRefreshClient<Target>
        + CanRaiseAsyncError<ErrorOf<Relay::TargetChain>>,
    Target: RelayTarget,
    Relay::TargetChain: CanQueryChainHeight,
{
    async fn auto_relay(
        relay: &Relay,
        target: Target,
        refresh_rate: Option<Duration>,
    ) -> Result<(), Relay::Error> {
        let start_height = relay
            .target_chain()
            .query_chain_height()
            .await
            .map_err(Relay::raise_error)?;

        let auto_relay_task = relay.auto_relay_with_heights(target, &start_height, None);

        if let Some(refresh_rate) = refresh_rate {
            let auto_refresh_task = relay.auto_refresh_client(target, refresh_rate, None);

            let _ = futures::join!(auto_relay_task, auto_refresh_task);
        } else {
            let _ = auto_relay_task.await;
        }

        Ok(())
    }
}

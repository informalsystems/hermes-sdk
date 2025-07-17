use core::time::Duration;

use cgp::core::error::ErrorOf;
use hermes_chain_components::traits::{CanQueryChainHeight, CanSendMessages};
use hermes_prelude::*;
use hermes_runtime_components::traits::{CanRunConcurrentTasks, CanSleep, HasRuntime};

use crate::relay::traits::{
    CanBuildTargetUpdateClientMessage, ClientRefresher, ClientRefresherComponent, HasTargetChains,
    RelayTarget,
};

#[cgp_new_provider(ClientRefresherComponent)]
impl<Relay, Target> ClientRefresher<Relay, Target> for RefreshClientWithInterval
where
    Relay: Clone
        + HasRuntime
        + HasTargetChains<Target>
        + CanBuildTargetUpdateClientMessage<Target>
        + CanRaiseAsyncError<ErrorOf<Relay::CounterpartyChain>>,
    Target: RelayTarget,
    Relay::TargetChain: CanQueryChainHeight + CanSendMessages,
    Relay::CounterpartyChain: CanQueryChainHeight,
    Relay::Runtime: CanRunConcurrentTasks + CanSleep,
{
    async fn auto_refresh_client(
        relay: &Relay,
        _target: Target,
        interval: Duration,
    ) -> Result<(), Relay::Error> {
        let counterparty_chain = relay.counterparty_chain();
        let runtime = relay.runtime();

        loop {
            let latest_height = counterparty_chain
                .query_chain_height()
                .await
                .map_err(Relay::raise_error)?;

            let update_message = relay
                .build_target_update_client_messages(Target::default(), &latest_height)
                .await
                .expect("failed to build update client message during refresh");
            let _ = relay.target_chain().send_messages(update_message).await;

            runtime.sleep(interval).await;
        }
    }
}

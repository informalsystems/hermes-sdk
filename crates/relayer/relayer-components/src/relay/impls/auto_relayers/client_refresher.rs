use core::time::Duration;

use cgp::core::error::ErrorOf;
use hermes_chain_components::traits::{CanQueryChainHeight, CanSendMessages};
use hermes_chain_components::types::aliases::HeightOf;
use hermes_logging_components::traits::CanLog;
use hermes_logging_components::types::LevelDebug;
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
        + CanLog<LevelDebug>
        + HasRuntime
        + HasTargetChains<Target>
        + CanBuildTargetUpdateClientMessage<Target>
        + CanRaiseAsyncError<ErrorOf<Relay::TargetChain>>
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
        end_height: Option<&HeightOf<Relay::TargetChain>>,
    ) -> Result<(), Relay::Error> {
        let chain = relay.target_chain();
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

            if let Some(end_height) = end_height {
                let latest_chain_height = chain
                    .query_chain_height()
                    .await
                    .map_err(Relay::raise_error)?;

                if *end_height <= latest_chain_height {
                    relay.log(&alloc::format!("Will stop auto refresher task as target chain is at a height equal or height than the configured stop height. \
                    {end_height} <= {latest_chain_height:?}"), &LevelDebug).await;

                    return Ok(());
                }
            }

            runtime.sleep(interval).await;
        }
    }
}

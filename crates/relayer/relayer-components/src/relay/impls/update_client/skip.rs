use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::HasMessageType;
use hermes_logging_components::traits::CanLog;

use crate::chain::traits::{
    CanQueryConsensusStateWithLatestHeight, HasConsensusStateType, HasHeightType,
};
use crate::chain::types::aliases::HeightOf;
use crate::relay::traits::{
    CounterpartyChainOf, HasTargetChainTypes, HasTargetChains, HasTargetClientIds, RelayTarget,
    TargetUpdateClientMessageBuilder, TargetUpdateClientMessageBuilderComponent,
};

pub struct LogSkipBuildUpdateClientMessage<'a, Relay, Target>
where
    Target: RelayTarget,
    Relay: HasTargetChainTypes<Target, CounterpartyChain: HasHeightType>,
{
    pub target_height: &'a HeightOf<CounterpartyChainOf<Relay, Target>>,
}

#[cgp_new_provider(TargetUpdateClientMessageBuilderComponent)]
impl<Relay, Target, InUpdateClient, TargetChain, CounterpartyChain>
    TargetUpdateClientMessageBuilder<Relay, Target> for SkipUpdateClient<InUpdateClient>
where
    Target: RelayTarget,
    Relay: HasTargetChains<Target, TargetChain = TargetChain, CounterpartyChain = CounterpartyChain>
        + HasTargetClientIds<Target>
        + for<'a> CanLog<LogSkipBuildUpdateClientMessage<'a, Relay, Target>>
        + HasAsyncErrorType,
    InUpdateClient: TargetUpdateClientMessageBuilder<Relay, Target>,
    CounterpartyChain: HasConsensusStateType<TargetChain> + HasHeightType,
    TargetChain: CanQueryConsensusStateWithLatestHeight<CounterpartyChain> + HasMessageType,
{
    async fn build_target_update_client_messages(
        relay: &Relay,
        target: Target,
        target_height: &HeightOf<Relay::CounterpartyChain>,
    ) -> Result<Vec<TargetChain::Message>, Relay::Error> {
        let target_chain = relay.target_chain();
        let target_client_id = relay.target_client_id();

        let consensus_state = target_chain
            .query_consensus_state_with_latest_height(PhantomData, target_client_id, target_height)
            .await;

        match consensus_state {
            Ok(_) => {
                relay.log(
                    "skip building update client message, as the target chain already has one at given height",
                    &LogSkipBuildUpdateClientMessage {
                        target_height,
                    }
                ).await;

                Ok(Vec::new())
            }
            Err(_) => {
                InUpdateClient::build_target_update_client_messages(relay, target, target_height)
                    .await
            }
        }
    }
}

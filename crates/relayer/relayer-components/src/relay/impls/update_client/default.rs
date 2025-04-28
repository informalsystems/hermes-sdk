use alloc::vec::Vec;

use hermes_chain_components::impls::CanWaitChainReachHeight;
use hermes_chain_components::traits::CanQueryConsensusStateWithLatestHeight;
use hermes_logging_components::traits::CanLog;
use hermes_prelude::*;

use crate::chain::traits::{
    CanBuildUpdateClientMessage, CanBuildUpdateClientPayload, CanQueryClientStateWithLatestHeight,
    CanQueryConsensusStateHeight, HasClientStateFields,
};
use crate::relay::impls::{
    BuildUpdateClientMessages, LogClientUpdateMessage, LogSkipBuildUpdateClientMessage,
    LogWaitUpdateClientHeightStatus, SkipUpdateClient, WaitUpdateClient,
};
use crate::relay::traits::{
    HasTargetChainTypes, HasTargetChains, HasTargetClientIds, RelayTarget,
    TargetUpdateClientMessageBuilder, TargetUpdateClientMessageBuilderComponent,
};

pub struct DefaultTargetUpdateClientMessageBuilder;

#[cgp_provider(TargetUpdateClientMessageBuilderComponent)]
impl<Relay, Target, TargetChain, CounterpartyChain> TargetUpdateClientMessageBuilder<Relay, Target>
    for DefaultTargetUpdateClientMessageBuilder
where
    Target: RelayTarget,
    Relay: HasTargetChainTypes<
            Target,
            TargetChain = TargetChain,
            CounterpartyChain = CounterpartyChain,
        > + HasTargetChains<Target>
        + HasTargetClientIds<Target>
        + for<'a> CanLog<LogWaitUpdateClientHeightStatus<'a, Relay, Target>>
        + for<'a> CanLog<LogSkipBuildUpdateClientMessage<'a, Relay, Target>>
        + for<'a> CanLog<LogClientUpdateMessage<'a, Relay, Target>>
        + CanRaiseAsyncError<TargetChain::Error>
        + CanRaiseAsyncError<CounterpartyChain::Error>,
    TargetChain: CanQueryClientStateWithLatestHeight<CounterpartyChain>
        + CanBuildUpdateClientMessage<CounterpartyChain>
        + CanQueryConsensusStateHeight<CounterpartyChain>
        + CanQueryConsensusStateWithLatestHeight<CounterpartyChain>,
    CounterpartyChain: CanWaitChainReachHeight
        + CanBuildUpdateClientPayload<TargetChain>
        + HasClientStateFields<TargetChain>,
    CounterpartyChain::Height: Clone,
{
    async fn build_target_update_client_messages(
        relay: &Relay,
        target: Target,
        height: &CounterpartyChain::Height,
    ) -> Result<Vec<TargetChain::Message>, Relay::Error> {
        <SkipUpdateClient<WaitUpdateClient<BuildUpdateClientMessages>>>::build_target_update_client_messages(relay, target, height).await
    }
}

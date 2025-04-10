use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_components::impls::wait_chain_reach_height::CanWaitChainReachHeight;
use hermes_chain_components::traits::queries::consensus_state::CanQueryConsensusStateWithLatestHeight;
use hermes_logging_components::traits::logger::CanLog;

use crate::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use crate::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::chain::traits::queries::consensus_state_height::CanQueryConsensusStateHeight;
use crate::chain::traits::types::client_state::HasClientStateFields;
use crate::relay::impls::update_client::build::{
    BuildUpdateClientMessages, LogClientUpdateMessage,
};
use crate::relay::impls::update_client::skip::{LogSkipBuildUpdateClientMessage, SkipUpdateClient};
use crate::relay::impls::update_client::wait::{LogWaitUpdateClientHeightStatus, WaitUpdateClient};
use crate::relay::traits::target::{
    HasTargetChainTypes, HasTargetChains, HasTargetClientIds, RelayTarget,
};
use crate::relay::traits::update_client_message_builder::{
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

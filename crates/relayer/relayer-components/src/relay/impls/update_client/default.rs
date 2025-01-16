use alloc::vec::Vec;

use cgp::prelude::CanRaiseAsyncError;
use hermes_chain_components::impls::wait_chain_reach_height::CanWaitChainReachHeight;
use hermes_chain_components::traits::queries::consensus_state::CanQueryConsensusStateWithLatestHeight;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;

use crate::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use crate::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::chain::traits::queries::consensus_state_height::CanQueryConsensusStateHeight;
use crate::chain::traits::types::client_state::HasClientStateFields;
use crate::relay::impls::update_client::build::BuildUpdateClientMessages;
use crate::relay::impls::update_client::skip::{LogSkipBuildUpdateClientMessage, SkipUpdateClient};
use crate::relay::impls::update_client::wait::{LogWaitUpdateClientHeightStatus, WaitUpdateClient};
use crate::relay::traits::target::{
    HasTargetChainTypes, HasTargetChains, HasTargetClientIds, RelayTarget,
};
use crate::relay::traits::update_client_message_builder::TargetUpdateClientMessageBuilder;

pub struct DefaultTargetUpdateClientMessageBuilder;

impl<Relay, Target, TargetChain, CounterpartyChain> TargetUpdateClientMessageBuilder<Relay, Target>
    for DefaultTargetUpdateClientMessageBuilder
where
    Target: RelayTarget,
    Relay: HasLogger
        + HasTargetChainTypes<
            Target,
            TargetChain = TargetChain,
            CounterpartyChain = CounterpartyChain,
        > + HasTargetChains<Target>
        + HasTargetClientIds<Target>
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
    Relay::Logger: for<'a> CanLog<LogWaitUpdateClientHeightStatus<'a, Relay, Target>>
        + for<'a> CanLog<LogSkipBuildUpdateClientMessage<'a, Relay, Target>>,
{
    async fn build_target_update_client_messages(
        relay: &Relay,
        target: Target,
        height: &CounterpartyChain::Height,
    ) -> Result<Vec<TargetChain::Message>, Relay::Error> {
        <SkipUpdateClient<WaitUpdateClient<BuildUpdateClientMessages>>>::build_target_update_client_messages(relay, target, height).await
    }
}

use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::*;

use crate::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use crate::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::chain::traits::queries::consensus_state_height::CanQueryConsensusStateHeight;
use crate::chain::traits::types::client_state::HasClientStateFields;
use crate::relay::traits::target::{
    HasTargetChainTypes, HasTargetChains, HasTargetClientIds, RelayTarget,
};
use crate::relay::traits::update_client_message_builder::{
    TargetUpdateClientMessageBuilder, TargetUpdateClientMessageBuilderComponent,
};

pub struct BuildUpdateClientMessages;

#[cgp_provider(TargetUpdateClientMessageBuilderComponent)]
impl<Relay, Target, TargetChain, CounterpartyChain> TargetUpdateClientMessageBuilder<Relay, Target>
    for BuildUpdateClientMessages
where
    Target: RelayTarget,
    Relay: HasTargetChainTypes<
            Target,
            TargetChain = TargetChain,
            CounterpartyChain = CounterpartyChain,
        > + HasTargetChains<Target>
        + HasTargetClientIds<Target>
        + CanRaiseAsyncError<TargetChain::Error>
        + CanRaiseAsyncError<CounterpartyChain::Error>,
    TargetChain: CanQueryClientStateWithLatestHeight<CounterpartyChain>
        + CanBuildUpdateClientMessage<CounterpartyChain>
        + CanQueryConsensusStateHeight<CounterpartyChain>,
    CounterpartyChain: CanBuildUpdateClientPayload<TargetChain> + HasClientStateFields<TargetChain>,
    CounterpartyChain::Height: Clone,
{
    async fn build_target_update_client_messages(
        relay: &Relay,
        _target: Target,
        target_height: &CounterpartyChain::Height,
    ) -> Result<Vec<TargetChain::Message>, Relay::Error> {
        let target_client_id = relay.target_client_id();

        let target_chain = relay.target_chain();
        let counterparty_chain = relay.counterparty_chain();

        let client_state = target_chain
            .query_client_state_with_latest_height(PhantomData, target_client_id)
            .await
            .map_err(Relay::raise_error)?;

        let client_state_height = CounterpartyChain::client_state_latest_height(&client_state);

        // If the client state height is already the same as target height, then there
        // is no need to build any UpdateClient message
        if &client_state_height == target_height {
            return Ok(Vec::new());
        }

        let trusted_height = if &client_state_height < target_height {
            // If the client state height is less than the target height, we can use that
            // as a base trust height to build our UpdateClient headers.
            client_state_height.clone()
        } else {
            // If the client state height is greater than the target height, it means we
            // have to find a previous consensus height that is less than the target height.
            let consensus_state_height = target_chain
                .find_consensus_state_height_before(target_client_id, target_height)
                .await
                .map_err(Relay::raise_error)?;

            // If we happen to find a consensus height that matches the target height,
            // then there is no need to build any UpdateClient message.
            if &consensus_state_height == target_height {
                return Ok(Vec::new());
            }

            consensus_state_height
        };

        let update_payload = counterparty_chain
            .build_update_client_payload(&trusted_height, target_height, client_state)
            .await
            .map_err(Relay::raise_error)?;

        let messages = target_chain
            .build_update_client_message(target_client_id, update_payload)
            .await
            .map_err(Relay::raise_error)?;

        Ok(messages)
    }
}

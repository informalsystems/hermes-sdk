use alloc::vec::Vec;

use cgp_core::async_trait;

use crate::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use crate::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use crate::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use crate::chain::traits::queries::consensus_state_height::CanQueryConsensusStateHeight;
use crate::chain::traits::types::client_state::HasClientStateFields;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::target::ChainTarget;
use crate::relay::traits::update_client_message_builder::UpdateClientMessageBuilder;

pub struct BuildUpdateClientMessages;

#[async_trait]
impl<Relay, Target, TargetChain, CounterpartyChain> UpdateClientMessageBuilder<Relay, Target>
    for BuildUpdateClientMessages
where
    Relay: HasRelayChains,
    Target: ChainTarget<Relay, TargetChain = TargetChain, CounterpartyChain = CounterpartyChain>,
    TargetChain: CanQueryClientStateWithLatestHeight<CounterpartyChain>
        + CanBuildUpdateClientMessage<CounterpartyChain>
        + CanQueryConsensusStateHeight<CounterpartyChain>,
    CounterpartyChain: CanBuildUpdateClientPayload<TargetChain> + HasClientStateFields<TargetChain>,
    CounterpartyChain::Height: Clone,
{
    async fn build_update_client_messages(
        relay: &Relay,
        _target: Target,
        target_height: &CounterpartyChain::Height,
    ) -> Result<Vec<TargetChain::Message>, Relay::Error> {
        let target_client_id = Target::target_client_id(relay);

        let target_chain = Target::target_chain(relay);
        let counterparty_chain = Target::counterparty_chain(relay);

        let client_state = target_chain
            .query_client_state_with_latest_height(target_client_id)
            .await
            .map_err(Target::target_chain_error)?;

        let client_state_height = CounterpartyChain::client_state_latest_height(&client_state);

        // If the client state height is already the same as target height, then there
        // is no need to build any UpdateClient message
        if client_state_height == target_height {
            return Ok(Vec::new());
        }

        let trusted_height = if client_state_height < target_height {
            // If the client state height is less than the target height, we can use that
            // as a base trust height to build our UpdateClient headers.
            client_state_height.clone()
        } else {
            // If the client state height is greater than the target height, it means we
            // have to find a previous consensus height that is less than the target height.
            let consensus_state_height = target_chain
                .find_consensus_state_height_before(target_client_id, target_height)
                .await
                .map_err(Target::target_chain_error)?;

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
            .map_err(Target::counterparty_chain_error)?;

        let messages = target_chain
            .build_update_client_message(target_client_id, update_payload)
            .await
            .map_err(Target::target_chain_error)?;

        Ok(messages)
    }
}

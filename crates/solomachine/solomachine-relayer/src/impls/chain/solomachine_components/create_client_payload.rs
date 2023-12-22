use async_trait::async_trait;
use hermes_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilder;

use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;
use crate::types::client_state::SolomachineClientState;
use crate::types::consensus_state::SolomachineConsensusState;
use crate::types::payloads::client::SolomachineCreateClientPayload;

pub struct BuildSolomachineCreateClientPayload;

#[async_trait]
impl<Chain, Counterparty> CreateClientPayloadBuilder<SolomachineChain<Chain>, Counterparty>
    for BuildSolomachineCreateClientPayload
where
    Chain: Solomachine,
{
    async fn build_create_client_payload(
        chain: &SolomachineChain<Chain>,
        _create_client_options: &(),
    ) -> Result<SolomachineCreateClientPayload, Chain::Error> {
        let public_key = chain.chain.public_key().clone();
        let diversifier = chain.chain.current_diversifier();
        let timestamp = chain.chain.current_time();

        let consensus_state = SolomachineConsensusState {
            public_key: Some(public_key),
            diversifier,
            timestamp,
        };

        let client_state = SolomachineClientState {
            sequence: 1,
            is_frozen: false,
            consensus_state,
        };

        let payload = SolomachineCreateClientPayload { client_state };

        Ok(payload)
    }
}

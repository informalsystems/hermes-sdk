use cgp::prelude::*;
use hermes_relayer_components::chain::traits::payload_builders::create_client::{
    CreateClientPayloadBuilder, CreateClientPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
};

use crate::traits::solomachine::Solomachine;
use crate::types::client_state::SolomachineClientState;
use crate::types::consensus_state::SolomachineConsensusState;
use crate::types::payloads::client::SolomachineCreateClientPayload;

pub struct BuildSolomachineCreateClientPayload;

#[cgp_provider(CreateClientPayloadBuilderComponent)]
impl<Chain, Counterparty> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildSolomachineCreateClientPayload
where
    Chain: Solomachine
        + HasCreateClientPayloadOptionsType<Counterparty>
        + HasCreateClientPayloadType<
            Counterparty,
            CreateClientPayload = SolomachineCreateClientPayload,
        > + HasAsyncErrorType,
{
    async fn build_create_client_payload(
        chain: &Chain,
        _create_client_options: &Chain::CreateClientPayloadOptions,
    ) -> Result<SolomachineCreateClientPayload, Chain::Error> {
        let public_key = chain.public_key().clone();
        let diversifier = chain.current_diversifier();
        let timestamp = chain.current_time();

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

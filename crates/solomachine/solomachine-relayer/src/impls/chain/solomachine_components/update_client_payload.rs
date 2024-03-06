use cgp_core::prelude::*;
use hermes_protobuf_components::types::Any;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_relayer_components::encode::types::via::Via;
use ibc_relayer_types::Height;

use crate::methods::encode::header_data::sign_header_data;
use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;
use crate::types::client_state::SolomachineClientState;
use crate::types::header::{SolomachineHeader, SolomachineHeaderData, SolomachineSignHeaderData};
use crate::types::payloads::client::SolomachineUpdateClientPayload;

pub struct BuildSolomachineUpdateClientPayload;

#[async_trait]
impl<Chain, Counterparty> UpdateClientPayloadBuilder<SolomachineChain<Chain>, Counterparty>
    for BuildSolomachineUpdateClientPayload
where
    Chain: Solomachine,
{
    async fn build_update_client_payload(
        chain: &SolomachineChain<Chain>,
        _trusted_height: &Height,
        _target_height: &Height,
        client_state: Via<Any, SolomachineClientState>,
    ) -> Result<SolomachineUpdateClientPayload, Chain::Error> {
        // TODO: check that the public key is the same in the consensus state.
        // We currently only support updating the diversifier but not the public key.

        let public_key = chain.chain.public_key();
        let current_diversifier = &client_state.value.consensus_state.diversifier;

        let next_diversifier = chain.chain.current_diversifier();

        // TODO: check that current time is greater than or equal to the consensus state time.
        let timestamp = chain.chain.current_time();

        let header_data = SolomachineHeaderData {
            new_public_key: public_key.clone(),
            new_diversifier: next_diversifier,
        };

        let sign_data = SolomachineSignHeaderData {
            header_data,
            sequence: client_state.value.sequence,
            timestamp,
            diversifier: current_diversifier.clone(),
        };

        let secret_key = chain.chain.secret_key();

        let signature = sign_header_data(secret_key, &sign_data).map_err(Chain::encode_error)?;

        let header = SolomachineHeader {
            timestamp,
            signature,
            header_data: sign_data.header_data,
        };

        let payload = SolomachineUpdateClientPayload { header };

        Ok(payload)
    }
}

use cgp::core::error::HasErrorType;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc::core::client::types::Height;

use crate::methods::encode::header_data::sign_header_data;
use crate::traits::solomachine::Solomachine;
use crate::types::client_state::SolomachineClientState;
use crate::types::header::{SolomachineHeader, SolomachineHeaderData, SolomachineSignHeaderData};
use crate::types::payloads::client::SolomachineUpdateClientPayload;

pub struct BuildSolomachineUpdateClientPayload;

impl<Chain, Counterparty> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildSolomachineUpdateClientPayload
where
    Chain: Solomachine
        + HasUpdateClientPayloadType<
            Counterparty,
            UpdateClientPayload = SolomachineUpdateClientPayload,
        > + HasClientStateType<Counterparty, ClientState = SolomachineClientState>
        + HasHeightType<Height = Height>
        + HasErrorType,
{
    async fn build_update_client_payload(
        chain: &Chain,
        _trusted_height: &Height,
        _target_height: &Height,
        client_state: SolomachineClientState,
    ) -> Result<SolomachineUpdateClientPayload, Chain::Error> {
        // TODO: check that the public key is the same in the consensus state.
        // We currently only support updating the diversifier but not the public key.

        let public_key = chain.public_key();
        let current_diversifier = &client_state.consensus_state.diversifier;

        let next_diversifier = chain.current_diversifier();

        // TODO: check that current time is greater than or equal to the consensus state time.
        let timestamp = chain.current_time();

        let header_data = SolomachineHeaderData {
            new_public_key: public_key.clone(),
            new_diversifier: next_diversifier,
        };

        let sign_data = SolomachineSignHeaderData {
            header_data,
            sequence: client_state.sequence,
            timestamp,
            diversifier: current_diversifier.clone(),
        };

        let secret_key = chain.secret_key();

        let signature = sign_header_data(secret_key, &sign_data);

        let header = SolomachineHeader {
            timestamp,
            signature,
            header_data: sign_data.header_data,
        };

        let payload = SolomachineUpdateClientPayload { header };

        Ok(payload)
    }
}

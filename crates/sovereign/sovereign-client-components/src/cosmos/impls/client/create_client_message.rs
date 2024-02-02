use cgp_core::HasErrorType;
use hermes_cosmos_client_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_client_components::types::messages::client::create::CosmosCreateClientMessage;
use hermes_cosmos_relayer::types::error::{BaseError, Error};
use hermes_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayloadType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_wasm_client_components::wasm::types::messages::client::consensus::WasmConsensusState;
use hermes_wasm_client_components::wasm::types::messages::client::state::WasmClientState;
use ibc::proto::tendermint::v1::ClientState as RawTmClientState;
use ibc::proto::tendermint::v1::ConsensusState as RawConsensusState;

use crate::sovereign::types::payloads::client::SovereignCreateClientPayload;

/**
   Build a message to create a Sovereign client on a Cosmos chain
*/
pub struct BuildCreateSovereignWasmClientMessageOnCosmos;

impl<Chain, Counterparty> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateSovereignWasmClientMessageOnCosmos
where
    Chain: HasMessageType<Message = CosmosMessage> + HasErrorType<Error = Error>,
    Counterparty:
        HasCreateClientPayloadType<Chain, CreateClientPayload = SovereignCreateClientPayload>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        payload: SovereignCreateClientPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let any_inner_client_state = ibc::proto::Protobuf::<RawTmClientState>::encode_vec(
            payload.celestia_payload.client_state,
        );
        let client_state = WasmClientState {
            data: any_inner_client_state,
            checksum: payload.code_hash.clone(),
            latest_height: payload.latest_height,
        };
        let any_client_state = client_state.encode_protobuf().map_err(BaseError::encode)?;

        let any_inner_consensus_state = ibc::proto::Protobuf::<RawConsensusState>::encode_vec(
            payload.celestia_payload.consensus_state,
        );
        let consensus_state = WasmConsensusState {
            data: any_inner_consensus_state,
        };
        let any_consensus_state = consensus_state
            .encode_protobuf()
            .map_err(BaseError::encode)?;

        let message = CosmosCreateClientMessage {
            client_state: any_client_state,
            consensus_state: any_consensus_state,
        };

        Ok(message.to_cosmos_message())
    }
}

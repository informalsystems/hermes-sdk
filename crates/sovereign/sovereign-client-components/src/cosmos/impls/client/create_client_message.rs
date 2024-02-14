use cgp_core::HasErrorType;
use hermes_cosmos_client_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_client_components::types::messages::client::create::CosmosCreateClientMessage;
use hermes_cosmos_relayer::types::error::{BaseError, Error};
use hermes_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayloadType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_wasm_client_components::wasm::types::messages::client::consensus::WasmConsensusState;
use hermes_wasm_client_components::wasm::types::messages::client::state::WasmClientState;
use ibc_core::primitives::ToProto;
use prost::Message;

use crate::sovereign::types::payloads::client::SovereignCreateClientPayload;

/**
   Build a message to create a Sovereign client on a Cosmos chain
*/
pub struct BuildCreateSovereignClientMessageOnCosmos;

impl<Chain, Counterparty> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateSovereignClientMessageOnCosmos
where
    Chain: HasMessageType<Message = CosmosMessage> + HasErrorType<Error = Error>,
    Counterparty:
        HasCreateClientPayloadType<Chain, CreateClientPayload = SovereignCreateClientPayload>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        payload: SovereignCreateClientPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let raw_client_state = <sov_celestia_client::types::client_state::ClientState as ToProto<
            sov_celestia_client::types::proto::v1::ClientState,
        >>::to_any(payload.client_state);
        let client_state = WasmClientState {
            data: raw_client_state.encode_to_vec(),
            checksum: payload.code_hash.clone(),
            latest_height: payload.latest_height,
        };
        let any_client_state = client_state.encode_protobuf().map_err(BaseError::encode)?;

        let raw_consensus_state =
            <sov_celestia_client::types::consensus_state::ConsensusState as ToProto<
                sov_celestia_client::types::proto::v1::ConsensusState,
            >>::to_any(payload.consensus_state);
        let consensus_state: WasmConsensusState = WasmConsensusState {
            data: raw_consensus_state.encode_to_vec(),
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

use cgp_core::{CanRaiseError, HasErrorType};
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::client::create::CosmosCreateClientMessage;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayloadType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_wasm_client_components::types::client_state::WasmClientState;
use hermes_wasm_client_components::types::consensus_state::WasmConsensusState;
use ibc::core::primitives::ToProto;
use ibc_proto::google::protobuf::Any;
use prost::{EncodeError, Message};

use crate::sovereign::types::client_state::SovereignClientState;
use crate::sovereign::types::payloads::client::SovereignCreateClientPayload;

/**
   Build a message to create a Sovereign client on a Cosmos chain
*/
pub struct BuildCreateSovereignClientMessageOnCosmos;

impl<Chain, Counterparty> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateSovereignClientMessageOnCosmos
where
    Chain: HasMessageType<Message = CosmosMessage> + HasErrorType + CanRaiseError<EncodeError>,
    Counterparty:
        HasCreateClientPayloadType<Chain, CreateClientPayload = SovereignCreateClientPayload>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        payload: SovereignCreateClientPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let raw_client_state = <SovereignClientState as ToProto<
            sov_celestia_client::types::proto::v1::ClientState,
        >>::to_any(payload.client_state);
        let client_state = WasmClientState {
            data: raw_client_state.encode_to_vec(),
            checksum: payload.code_hash.clone(),
            latest_height: payload.latest_height,
        };
        let new_any_client_state = client_state.encode_protobuf().map_err(Chain::raise_error)?;

        let any_client_state = Any {
            type_url: new_any_client_state.type_url,
            value: new_any_client_state.value,
        };

        let raw_consensus_state =
            <sov_celestia_client::types::consensus_state::SovTmConsensusState as ToProto<
                sov_celestia_client::types::proto::v1::ConsensusState,
            >>::to_any(payload.consensus_state);
        let consensus_state: WasmConsensusState = WasmConsensusState {
            data: raw_consensus_state.encode_to_vec(),
        };
        let new_any_consensus_state = consensus_state
            .encode_protobuf()
            .map_err(Chain::raise_error)?;

        let any_consensus_state = Any {
            type_url: new_any_consensus_state.type_url,
            value: new_any_consensus_state.value,
        };

        let message = CosmosCreateClientMessage {
            client_state: any_client_state,
            consensus_state: any_consensus_state,
        };

        Ok(message.to_cosmos_message())
    }
}

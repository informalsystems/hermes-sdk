use cgp_core::CanRaiseError;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::client::create::CosmosCreateClientMessage;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::encoder::CanEncode;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_protobuf_encoding_components::types::Any;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayloadType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_wasm_client_components::types::client_state::WasmClientState;
use hermes_wasm_client_components::types::consensus_state::WasmConsensusState;
use ibc::core::primitives::ToProto;
use ibc_proto::google::protobuf::Any as ProtoAny;
use prost::{EncodeError, Message};

use crate::sovereign::types::client_state::SovereignClientState;
use crate::sovereign::types::consensus_state::SovereignConsensusState;
use crate::sovereign::types::payloads::client::SovereignCreateClientPayload;

/**
   Build a message to create a Sovereign client on a Cosmos chain
*/
pub struct BuildCreateSovereignClientMessageOnCosmos;

impl<Chain, Counterparty, Encoding> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateSovereignClientMessageOnCosmos
where
    Chain: HasMessageType<Message = CosmosMessage>
        + CanRaiseError<EncodeError>
        + CanRaiseError<Encoding::Error>,
    Counterparty: HasDefaultEncoding<Encoding = Encoding>
        + HasCreateClientPayloadType<Chain, CreateClientPayload = SovereignCreateClientPayload>,
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanConvert<WasmClientState, Any>
        + CanEncode<WasmClientState>
        + CanEncode<SovereignClientState>
        + CanEncode<SovereignConsensusState>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        payload: SovereignCreateClientPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let encoding = Counterparty::default_encoding();

        let sov_client_state_bytes = encoding
            .encode(&payload.client_state)
            .map_err(Chain::raise_error)?;

        let wasm_client_state = WasmClientState {
            data: sov_client_state_bytes,
            checksum: payload.code_hash.clone(),
            latest_height: payload.latest_height,
        };

        let wasm_client_state_any = encoding
            .convert(&wasm_client_state)
            .map_err(Chain::raise_error)?;

        let raw_consensus_state =
            <sov_celestia_client::types::consensus_state::SovTmConsensusState as ToProto<
                sov_celestia_client::types::proto::tendermint::v1::ConsensusState,
            >>::to_any(payload.consensus_state);

        let consensus_state: WasmConsensusState = WasmConsensusState {
            data: raw_consensus_state.encode_to_vec(),
        };
        let new_any_consensus_state = consensus_state
            .encode_protobuf()
            .map_err(Chain::raise_error)?;

        let any_consensus_state = ProtoAny {
            type_url: new_any_consensus_state.type_url,
            value: new_any_consensus_state.value,
        };

        let message = CosmosCreateClientMessage {
            client_state: ProtoAny {
                type_url: wasm_client_state_any.type_url,
                value: wasm_client_state_any.value,
            },
            consensus_state: any_consensus_state,
        };

        Ok(message.to_cosmos_message())
    }
}

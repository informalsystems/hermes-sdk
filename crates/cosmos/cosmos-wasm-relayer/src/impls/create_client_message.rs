use hermes_core::encoding_components::traits::{
    CanConvert, CanEncode, HasDefaultEncoding, HasEncodedType,
};
use hermes_core::encoding_components::types::AsBytes;
use hermes_core::relayer_components::chain::traits::{
    CreateClientMessageBuilder, CreateClientMessageBuilderComponent,
    HasCreateClientMessageOptionsType, HasCreateClientPayloadType, HasMessageType,
};
use hermes_cosmos_chain_components::traits::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::{
    CosmosCreateClientMessage, CosmosCreateClientPayload, TendermintClientState,
    TendermintConsensusState,
};
use hermes_prelude::*;
use hermes_protobuf_encoding_components::types::strategy::ViaAny;
use hermes_wasm_encoding_components::types::{WasmClientState, WasmConsensusState};
use ibc::core::client::types::Height;
use prost_types::Any;

use crate::types::CreateWasmTendermintMessageOptions;

pub struct BuildCreateWasmTendermintClientMessage;

#[cgp_provider(CreateClientMessageBuilderComponent)]
impl<Chain, Counterparty, Encoding> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateWasmTendermintClientMessage
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasCreateClientMessageOptionsType<
            Counterparty,
            CreateClientMessageOptions = CreateWasmTendermintMessageOptions,
        > + CanRaiseAsyncError<Encoding::Error>,
    Counterparty: HasCreateClientPayloadType<Chain, CreateClientPayload = CosmosCreateClientPayload>
        + HasDefaultEncoding<AsBytes, Encoding = Encoding>,
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanConvert<WasmClientState, Any>
        + CanConvert<WasmConsensusState, Any>
        + CanEncode<ViaAny, TendermintClientState>
        + CanEncode<ViaAny, TendermintConsensusState>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        options: &CreateWasmTendermintMessageOptions,
        payload: CosmosCreateClientPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        let encoding = Counterparty::default_encoding();

        let tm_client_state_bytes = encoding
            .encode(&payload.client_state)
            .map_err(Chain::raise_error)?;

        let latest_height = payload.client_state.latest_height;

        let wasm_client_state = WasmClientState {
            data: tm_client_state_bytes,
            checksum: options.code_hash.clone(),
            latest_height: Height::new(
                latest_height.revision_number(),
                latest_height.revision_height(),
            )
            .unwrap(),
        };

        let wasm_client_state_any = encoding
            .convert(&wasm_client_state)
            .map_err(Chain::raise_error)?;

        let tm_consensus_state_bytes = encoding
            .encode(&payload.consensus_state)
            .map_err(Chain::raise_error)?;

        let wasm_consensus_state = WasmConsensusState {
            data: tm_consensus_state_bytes,
        };

        let wasm_consensus_state_any = encoding
            .convert(&wasm_consensus_state)
            .map_err(Chain::raise_error)?;

        let message = CosmosCreateClientMessage {
            client_state: wasm_client_state_any,
            consensus_state: wasm_consensus_state_any,
        };

        Ok(message.to_cosmos_message())
    }
}

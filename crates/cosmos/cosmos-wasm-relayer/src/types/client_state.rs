use hermes_core::encoding_components::traits::{
    CanConvert, CanDecode, CanEncode, Converter, ConverterComponent, HasEncodedType,
};
use hermes_cosmos_core::chain_components::types::TendermintClientState;
use hermes_cosmos_core::protobuf_encoding_components::types::strategy::ViaAny;
use hermes_cosmos_core::wasm_encoding_components::types::WasmClientState;
use hermes_prelude::*;
use ibc::core::client::types::Height;
use prost_types::Any;

pub struct WasmTendermintClientState {
    pub tendermint_client_state: TendermintClientState,
    pub wasm_code_hash: Vec<u8>,
}

impl From<WasmTendermintClientState> for TendermintClientState {
    fn from(value: WasmTendermintClientState) -> Self {
        value.tendermint_client_state
    }
}

pub struct EncodeWasmTendermintClientState;

#[cgp_provider(ConverterComponent)]
impl<Encoding> Converter<Encoding, WasmTendermintClientState, Any>
    for EncodeWasmTendermintClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<ViaAny, TendermintClientState>
        + CanConvert<WasmClientState, Any>,
{
    fn convert(
        encoding: &Encoding,
        client_state: &WasmTendermintClientState,
    ) -> Result<Any, Encoding::Error> {
        let tendermint_client_state_bytes =
            encoding.encode(&client_state.tendermint_client_state)?;

        let latest_height = client_state.tendermint_client_state.latest_height;

        let wasm_client_state = WasmClientState {
            data: tendermint_client_state_bytes,
            checksum: client_state.wasm_code_hash.clone(),
            latest_height: Height::new(
                latest_height.revision_number(),
                latest_height.revision_height(),
            )
            .unwrap(),
        };

        encoding.convert(&wasm_client_state)
    }
}

#[cgp_provider(ConverterComponent)]
impl<Encoding> Converter<Encoding, Any, WasmTendermintClientState>
    for EncodeWasmTendermintClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanDecode<ViaAny, TendermintClientState>
        + CanConvert<Any, WasmClientState>,
{
    fn convert(
        encoding: &Encoding,
        client_state_any: &Any,
    ) -> Result<WasmTendermintClientState, Encoding::Error> {
        let wasm_client_state = encoding.convert(client_state_any)?;

        let tendermint_client_state = encoding.decode(&wasm_client_state.data)?;

        let wrapped_tendermint_client_state = WasmTendermintClientState {
            tendermint_client_state,
            wasm_code_hash: wasm_client_state.checksum,
        };

        Ok(wrapped_tendermint_client_state)
    }
}

use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_encoding_components::traits::convert::{CanConvert, Converter};
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_protobuf_encoding_components::types::strategy::ViaAny;
use hermes_wasm_encoding_components::types::client_state::WasmClientState;
use ibc::core::client::types::Height;
use prost_types::Any;

pub struct WrappedTendermintClientState {
    pub tendermint_client_state: TendermintClientState,
    pub wasm_code_hash: Vec<u8>,
}

impl From<WrappedTendermintClientState> for TendermintClientState {
    fn from(value: WrappedTendermintClientState) -> Self {
        value.tendermint_client_state
    }
}

pub struct EncodeWrappedTendermintClientState;

impl<Encoding> Converter<Encoding, WrappedTendermintClientState, Any>
    for EncodeWrappedTendermintClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<ViaAny, TendermintClientState>
        + CanConvert<WasmClientState, Any>,
{
    fn convert(
        encoding: &Encoding,
        client_state: &WrappedTendermintClientState,
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

impl<Encoding> Converter<Encoding, Any, WrappedTendermintClientState>
    for EncodeWrappedTendermintClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanDecode<ViaAny, TendermintClientState>
        + CanConvert<Any, WasmClientState>,
{
    fn convert(
        encoding: &Encoding,
        client_state_any: &Any,
    ) -> Result<WrappedTendermintClientState, Encoding::Error> {
        let wasm_client_state = encoding.convert(client_state_any)?;

        let tendermint_client_state = encoding.decode(&wasm_client_state.data)?;

        let wrapped_tendermint_client_state = WrappedTendermintClientState {
            tendermint_client_state,
            wasm_code_hash: wasm_client_state.checksum,
        };

        Ok(wrapped_tendermint_client_state)
    }
}

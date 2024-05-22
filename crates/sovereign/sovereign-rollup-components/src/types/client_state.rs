use hermes_encoding_components::traits::convert::{CanConvert, Converter};
use hermes_encoding_components::traits::decoder::CanDecode;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::encoder::CanEncode;
use hermes_protobuf_encoding_components::types::Any;
use hermes_wasm_client_components::types::client_state::WasmClientState;
pub use sov_celestia_client::types::client_state::SovTmClientState as SovereignClientState;
pub use sov_celestia_client::types::proto::v1::ClientState as ProtoSovereignClientState;

#[derive(Debug)]
pub struct WrappedSovereignClientState {
    pub sovereign_client_state: SovereignClientState,
    pub wasm_code_hash: Vec<u8>,
}

pub struct EncodeWrappedSovereignClientState;

impl<Encoding> Converter<Encoding, WrappedSovereignClientState, Any>
    for EncodeWrappedSovereignClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<Any, SovereignClientState>
        + CanConvert<WasmClientState, Any>,
{
    fn convert(
        encoding: &Encoding,
        client_state: &WrappedSovereignClientState,
    ) -> Result<Any, Encoding::Error> {
        let sovereign_client_state_bytes = encoding.encode(&client_state.sovereign_client_state)?;

        let wasm_client_state = WasmClientState {
            data: sovereign_client_state_bytes,
            checksum: client_state.wasm_code_hash.clone(),
            latest_height: client_state
                .sovereign_client_state
                .sovereign_params
                .latest_height,
        };

        encoding.convert(&wasm_client_state)
    }
}

impl<Encoding> Converter<Encoding, Any, WrappedSovereignClientState>
    for EncodeWrappedSovereignClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanDecode<Any, SovereignClientState>
        + CanConvert<Any, WasmClientState>,
{
    fn convert(
        encoding: &Encoding,
        client_state_any: &Any,
    ) -> Result<WrappedSovereignClientState, Encoding::Error> {
        let wasm_client_state = encoding.convert(client_state_any)?;

        let sovereign_client_state = encoding.decode(&wasm_client_state.data)?;

        let wrapped_sovereign_client_state = WrappedSovereignClientState {
            sovereign_client_state,
            wasm_code_hash: wasm_client_state.checksum,
        };

        Ok(wrapped_sovereign_client_state)
    }
}

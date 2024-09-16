use hermes_encoding_components::traits::convert::{CanConvert, Converter};
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::ViaAny;

use crate::types::client_state::WasmClientState;

pub struct DecodeViaWasmClientState;

impl<Encoding, Value> Converter<Encoding, Any, Value> for DecodeViaWasmClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanConvert<Any, WasmClientState>
        + CanDecode<ViaAny, Value>,
{
    fn convert(encoding: &Encoding, any: &Any) -> Result<Value, Encoding::Error> {
        let wasm_client_state = encoding.convert(any)?;

        let value: Value = encoding.decode(&wasm_client_state.data)?;

        Ok(value)
    }
}

impl<Encoding, Value> Decoder<Encoding, WasmClientState, Value> for DecodeViaWasmClientState
where
    Encoding:
        HasEncodedType<Encoded = Vec<u8>> + CanDecode<Any, WasmClientState> + CanDecode<Any, Value>,
{
    fn decode(encoding: &Encoding, encoded: &Vec<u8>) -> Result<Value, Encoding::Error> {
        let wasm_client_state: WasmClientState = encoding.decode(encoded)?;

        let value: Value = encoding.decode(&wasm_client_state.data)?;

        Ok(value)
    }
}

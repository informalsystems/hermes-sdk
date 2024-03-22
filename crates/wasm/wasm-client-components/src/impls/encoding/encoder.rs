use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_encoding_components::traits::decoder::{CanDecode, Decoder};
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::types::via::Via;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::Any;

use crate::types::client_state::{ProtoWasmClientState, WasmClientState};

pub struct WasmEncoderComponents;

delegate_components! {
    WasmEncoderComponents {
        Via<Any, WasmClientState>: EncodeViaAny,
        WasmClientState: ConvertAndEncode<ProtoWasmClientState>,
        ProtoWasmClientState: EncodeAsProtobuf,
    }
}

pub struct EncodeViaWasmClientState;

impl<Encoding, Value> Decoder<Encoding, Via<WasmClientState, Value>> for EncodeViaWasmClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanDecode<Via<Any, WasmClientState>>
        + CanDecode<Via<Any, Value>>,
{
    fn decode(
        encoding: &Encoding,
        encoded: &Vec<u8>,
    ) -> Result<Via<WasmClientState, Value>, Encoding::Error> {
        let wasm_client_state: Via<Any, WasmClientState> = encoding.decode(encoded)?;

        let value: Via<Any, Value> = encoding.decode(&wasm_client_state.value.data)?;

        Ok(value.value.into())
    }
}

use cgp_core::prelude::*;
use hermes_protobuf_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_components::types::Any;
use hermes_relayer_components::encode::impls::convert_and_encode::ConvertAndEncode;
use hermes_relayer_components::encode::traits::decoder::{CanDecode, Decoder};
use hermes_relayer_components::encode::traits::encoded::HasEncodedType;
use hermes_relayer_components::encode::types::via::Via;

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

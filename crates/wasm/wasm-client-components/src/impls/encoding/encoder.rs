use cgp_core::prelude::*;
use hermes_protobuf_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_components::impls::wrap_any::EncodeWrapAny;
use hermes_protobuf_components::types::Any;
use hermes_relayer_components::encode::impls::convert_and_encode::ConvertAndEncode;
use hermes_relayer_components::encode::traits::decoder::{CanDecode, Decoder};
use hermes_relayer_components::encode::traits::encoded::HasEncodedType;
use hermes_relayer_components::encode::types::wrap::Wrap;

use crate::types::client_state::{ProtoWasmClientState, WasmClientState};

pub struct WasmEncoderComponents;

delegate_components! {
    WasmEncoderComponents {
        Wrap<Any, WasmClientState>: EncodeWrapAny,
        WasmClientState: ConvertAndEncode<ProtoWasmClientState>,
        ProtoWasmClientState: EncodeAsProtobuf,
    }
}

pub struct EncodeWrapWasmClientState;

impl<Encoding, Value> Decoder<Encoding, Wrap<WasmClientState, Value>> for EncodeWrapWasmClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanDecode<Wrap<Any, WasmClientState>>
        + CanDecode<Value>,
{
    fn decode(
        encoding: &Encoding,
        encoded: &Vec<u8>,
    ) -> Result<Wrap<WasmClientState, Value>, Encoding::Error> {
        let wasm_client_state: Wrap<Any, WasmClientState> = encoding.decode(encoded)?;

        let value: Value = encoding.decode(&wasm_client_state.value.data)?;

        Ok(value.into())
    }
}

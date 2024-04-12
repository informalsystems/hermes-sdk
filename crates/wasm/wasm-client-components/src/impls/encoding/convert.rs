use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert::{ConvertFrom, TryConvertFrom};
use hermes_protobuf_encoding_components::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use hermes_protobuf_encoding_components::impls::from_context::EncodeFromContext;
use hermes_protobuf_encoding_components::types::Any;

use crate::types::client_state::{ProtoWasmClientState, WasmClientState};

pub struct WasmConverterComponents;

delegate_components! {
    WasmConverterComponents {
        (WasmClientState, ProtoWasmClientState): ConvertFrom,
        (ProtoWasmClientState, WasmClientState): TryConvertFrom,
        (WasmClientState, Any): EncodeAsAnyProtobuf<EncodeFromContext>,
        (Any, WasmClientState): DecodeAsAnyProtobuf<EncodeFromContext>,
    }
}

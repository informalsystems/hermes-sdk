use cgp_core::prelude::*;
use hermes_protobuf_encoding_components::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use hermes_protobuf_encoding_components::impls::from_context::EncodeFromContext;
use hermes_protobuf_encoding_components::types::Any;

use crate::types::client_state::{
    ProtoConvertWasmClientState, ProtoWasmClientState, WasmClientState,
};

pub struct WasmConverterComponents;

delegate_components! {
    WasmConverterComponents {
        (WasmClientState, ProtoWasmClientState): ProtoConvertWasmClientState,
        (ProtoWasmClientState, WasmClientState): ProtoConvertWasmClientState,
        (WasmClientState, Any): EncodeAsAnyProtobuf<EncodeFromContext>,
        (Any, WasmClientState): DecodeAsAnyProtobuf<EncodeFromContext>,
    }
}

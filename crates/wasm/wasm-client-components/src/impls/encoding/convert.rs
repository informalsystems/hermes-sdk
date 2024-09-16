use cgp::prelude::*;
use hermes_encoding_components::impls::with_context::EncodeWithContext;
use hermes_protobuf_encoding_components::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;

use crate::types::client_state::WasmClientState;
use crate::types::consensus_state::WasmConsensusState;

pub struct WasmConverterComponents;

delegate_components! {
    WasmConverterComponents {
        (WasmClientState, Any): EncodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,
        (Any, WasmClientState): DecodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,
        (WasmConsensusState, Any): EncodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,
        (Any, WasmConsensusState): DecodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,
    }
}

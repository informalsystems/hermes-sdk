use cgp::prelude::*;
use hermes_encoding_components::impls::from_context::EncodeFromContext;
use hermes_protobuf_encoding_components::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;

use crate::types::client_state::WasmClientState;
use crate::types::consensus_state::{
    ProtoConvertWasmConsensusState, ProtoWasmConsensusState, WasmConsensusState,
};

pub struct WasmConverterComponents;

delegate_components! {
    WasmConverterComponents {
        (WasmConsensusState, ProtoWasmConsensusState): ProtoConvertWasmConsensusState,
        (ProtoWasmConsensusState, WasmConsensusState): ProtoConvertWasmConsensusState,
        (WasmClientState, Any): EncodeAsAnyProtobuf<ViaProtobuf, EncodeFromContext>,
        (Any, WasmClientState): DecodeAsAnyProtobuf<ViaProtobuf, EncodeFromContext>,
        (WasmConsensusState, Any): EncodeAsAnyProtobuf<ViaProtobuf, EncodeFromContext>,
        (Any, WasmConsensusState): DecodeAsAnyProtobuf<ViaProtobuf, EncodeFromContext>,
    }
}

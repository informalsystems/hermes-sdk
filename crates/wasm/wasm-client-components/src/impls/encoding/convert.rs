use cgp_core::prelude::*;
use hermes_protobuf_encoding_components::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use hermes_protobuf_encoding_components::impls::from_context::EncodeFromContext;
use hermes_protobuf_encoding_components::types::{Any, Protobuf};

use crate::types::client_state::{
    ProtoConvertWasmClientState, ProtoWasmClientState, WasmClientState,
};
use crate::types::consensus_state::{
    ProtoConvertWasmConsensusState, ProtoWasmConsensusState, WasmConsensusState,
};

pub struct WasmConverterComponents;

delegate_components! {
    WasmConverterComponents {
        (WasmClientState, ProtoWasmClientState): ProtoConvertWasmClientState,
        (ProtoWasmClientState, WasmClientState): ProtoConvertWasmClientState,
        (WasmConsensusState, ProtoWasmConsensusState): ProtoConvertWasmConsensusState,
        (ProtoWasmConsensusState, WasmConsensusState): ProtoConvertWasmConsensusState,
        (WasmClientState, Any): EncodeAsAnyProtobuf<Protobuf, EncodeFromContext>,
        (Any, WasmClientState): DecodeAsAnyProtobuf<Protobuf, EncodeFromContext>,
        (WasmConsensusState, Any): EncodeAsAnyProtobuf<Protobuf, EncodeFromContext>,
        (Any, WasmConsensusState): DecodeAsAnyProtobuf<Protobuf, EncodeFromContext>,
    }
}

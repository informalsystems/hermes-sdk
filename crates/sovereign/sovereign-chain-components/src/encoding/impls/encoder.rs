use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_encoding_components::types::via::Via;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::Any;
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::{
    DecodeViaWasmClientState, ProtoWasmClientState, WasmClientState,
};
use hermes_wasm_client_components::types::consensus_state::{
    EncodeViaWasmConsensusState, ProtoWasmConsensusState, WasmConsensusState,
};

use crate::sovereign::types::client_state::{ProtoSovereignClientState, SovereignClientState};
use crate::sovereign::types::consensus_state::{
    ProtoSovereignConsensusState, SovereignConsensusState,
};

pub struct SovereignEncoderComponents;

delegate_components! {
    SovereignEncoderComponents {
        [
            Via<Any, WasmClientState>,
            WasmClientState,
            ProtoWasmClientState,
            Via<Any, WasmConsensusState>,
            WasmConsensusState,
            ProtoWasmConsensusState,
        ]:
            WasmEncodingComponents,

        Via<Any, SovereignClientState>:
            EncodeViaAny,
        SovereignClientState:
            ConvertAndEncode<ProtoSovereignClientState>,
        ProtoSovereignClientState:
            EncodeAsProtobuf,

        Via<Any, SovereignConsensusState>:
            EncodeViaAny,
        SovereignConsensusState:
            ConvertAndEncode<ProtoSovereignConsensusState>,
        ProtoSovereignConsensusState:
            EncodeAsProtobuf,

        Via<WasmClientState, SovereignClientState>:
            DecodeViaWasmClientState,
        Via<WasmConsensusState, SovereignConsensusState>:
            EncodeViaWasmConsensusState,
    }
}

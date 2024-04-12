use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_encoding_components::types::via::Via;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::Any;
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::{
    EncodeViaWasmClientState, ProtoWasmClientState, WasmClientState,
};
use hermes_wasm_client_components::types::consensus_state::{
    ProtoWasmConsensusState, WasmConsensusState,
};
use sov_celestia_client::types::proto::tendermint::v1::ClientState as ProtoSovereignClientState;

use crate::sovereign::types::client_state::SovereignClientState;

pub struct SovereignEncoderComponents;

delegate_components! {
    SovereignEncoderComponents {
        Via<WasmClientState, SovereignClientState>: EncodeViaWasmClientState,
        Via<Any, SovereignClientState>: EncodeViaAny,
        SovereignClientState: ConvertAndEncode<ProtoSovereignClientState>,
        ProtoSovereignClientState: EncodeAsProtobuf,
        [
            Via<Any, WasmClientState>,
            WasmClientState,
            ProtoWasmClientState,
            Via<Any, WasmConsensusState>,
            WasmConsensusState,
            ProtoWasmConsensusState,
        ]:
            WasmEncodingComponents,
    }
}

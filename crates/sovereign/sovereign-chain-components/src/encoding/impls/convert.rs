use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert::{ConvertFrom, TryConvertFrom};
use hermes_protobuf_encoding_components::types::Any;
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::{ProtoWasmClientState, WasmClientState};
use hermes_wasm_client_components::types::consensus_state::{
    ProtoWasmConsensusState, WasmConsensusState,
};
use sov_celestia_client::types::proto::tendermint::v1::ClientState as ProtoSovereignClientState;

use crate::sovereign::types::client_state::SovereignClientState;

pub struct SovereignConverterComponents;

delegate_components! {
    SovereignConverterComponents {
        [
            (WasmClientState, ProtoWasmClientState),
            (ProtoWasmClientState, WasmClientState),
            (WasmConsensusState, ProtoWasmConsensusState),
            (ProtoWasmConsensusState, WasmConsensusState),
            (WasmClientState, Any),
            (Any, WasmClientState),
            (WasmConsensusState, Any),
            (Any, WasmConsensusState),
        ]:
            WasmEncodingComponents,
        (SovereignClientState, ProtoSovereignClientState):
            ConvertFrom,
        (ProtoSovereignClientState, SovereignClientState):
            TryConvertFrom,
    }
}

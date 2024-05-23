use cgp_core::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_cosmos_chain_components::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_encoding_components::impls::convert::{ConvertFrom, TryConvertFrom};
use hermes_protobuf_encoding_components::types::Any;
use hermes_sovereign_rollup_components::types::client_state::{
    EncodeWrappedSovereignClientState, ProtoSovereignClientState, SovereignClientState,
    WrappedSovereignClientState,
};
use hermes_sovereign_rollup_components::types::consensus_state::{
    ProtoSovereignConsensusState, SovereignConsensusState,
};
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::{ProtoWasmClientState, WasmClientState};
use hermes_wasm_client_components::types::consensus_state::{
    DecodeViaWasmConsensusState, EncodeViaWasmConsensusState, ProtoWasmConsensusState,
    WasmConsensusState,
};

pub struct SovereignConverterComponents;

delegate_components! {
    SovereignConverterComponents {
        [
            (TendermintClientState, ProtoTendermintClientState),
            (ProtoTendermintClientState, TendermintClientState),
            (TendermintConsensusState, ProtoTendermintConsensusState),
            (ProtoTendermintConsensusState, TendermintConsensusState),
            (TendermintClientState, Any),
            (Any, TendermintClientState),
            (TendermintConsensusState, Any),
            (Any, TendermintConsensusState),
        ]:
            CosmosEncodingComponents,

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

        (SovereignConsensusState, ProtoSovereignConsensusState):
            ConvertFrom,

        (ProtoSovereignConsensusState, SovereignConsensusState):
            TryConvertFrom,

        (SovereignConsensusState, Any):
            EncodeViaWasmConsensusState,

        (Any, SovereignConsensusState):
            DecodeViaWasmConsensusState,

        [
            (Any, WrappedSovereignClientState),
            (WrappedSovereignClientState, Any),
        ]:
            EncodeWrappedSovereignClientState
    }
}

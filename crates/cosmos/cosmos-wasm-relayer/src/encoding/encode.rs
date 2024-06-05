use cgp_core::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_protobuf_encoding_components::types::Protobuf;
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::{ProtoWasmClientState, WasmClientState};
use hermes_wasm_client_components::types::consensus_state::{
    ProtoWasmConsensusState, WasmConsensusState,
};
use prost_types::Any;

use hermes_cosmos_chain_components::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};

pub struct WasmCosmosEncoderComponents;

delegate_components! {
    WasmCosmosEncoderComponents {
        [
            (Any, TendermintClientState),
            (Protobuf, TendermintClientState),
            (Protobuf, ProtoTendermintClientState),
            (Any, TendermintConsensusState),
            (Protobuf,TendermintConsensusState),
            (Protobuf, ProtoTendermintConsensusState),
            (Protobuf, Any),
        ]:
            CosmosEncodingComponents,
        [
            (Any, WasmClientState),
            (Protobuf, WasmClientState),
            (Protobuf, ProtoWasmClientState),
            (Any, WasmConsensusState),
            (Protobuf, WasmConsensusState),
            (Protobuf, ProtoWasmConsensusState),
        ]:
            WasmEncodingComponents,
    }
}

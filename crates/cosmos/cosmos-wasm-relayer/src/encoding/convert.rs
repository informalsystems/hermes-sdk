use cgp::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_cosmos_chain_components::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::{ProtoWasmClientState, WasmClientState};
use hermes_wasm_client_components::types::consensus_state::{
    DecodeViaWasmConsensusState, EncodeViaWasmConsensusState, ProtoWasmConsensusState,
    WasmConsensusState,
};
use ibc::core::commitment_types::merkle::MerkleProof;
use ibc_proto::ibc::core::commitment::v1::MerkleProof as ProtoMerkleProof;
use prost_types::Any;

use crate::types::client_state::{
    EncodeWrappedTendermintClientState, WrappedTendermintClientState,
};

pub struct WasmCosmosConverterComponents;

delegate_components! {
    WasmCosmosConverterComponents {
        [
            (TendermintClientState, ProtoTendermintClientState),
            (ProtoTendermintClientState, TendermintClientState),
            (TendermintConsensusState, ProtoTendermintConsensusState),
            (ProtoTendermintConsensusState, TendermintConsensusState),
            (MerkleProof, ProtoMerkleProof),
            (ProtoMerkleProof, MerkleProof),
            (TendermintClientState, Any),
            (Any, TendermintClientState),
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

        [
            (Any, WrappedTendermintClientState),
            (WrappedTendermintClientState, Any),
        ]:
            EncodeWrappedTendermintClientState,

        (TendermintConsensusState, Any):
            EncodeViaWasmConsensusState,

        (Any, TendermintConsensusState):
            DecodeViaWasmConsensusState,

    }
}

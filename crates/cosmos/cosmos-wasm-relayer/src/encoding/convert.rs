use hermes_cosmos_core::chain_components::encoding::CosmosClientEncodingComponents;
use hermes_cosmos_core::chain_components::types::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_cosmos_core::wasm_encoding_components::components::WasmEncodingComponents;
use hermes_cosmos_core::wasm_encoding_components::impls::{
    DecodeViaWasmConsensusState, EncodeViaWasmConsensusState,
};
use hermes_cosmos_core::wasm_encoding_components::types::{
    WasmClientMessage, WasmClientState, WasmConsensusState,
};
use hermes_prelude::*;
use ibc::core::commitment_types::merkle::MerkleProof;
use ibc_proto::ibc::core::commitment::v1::MerkleProof as ProtoMerkleProof;
use prost_types::Any;

use crate::types::{EncodeWasmTendermintClientState, WasmTendermintClientState};

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
            CosmosClientEncodingComponents::Provider,
        [
            (WasmClientState, Any),
            (WasmConsensusState, Any),
            (WasmClientMessage, Any),

            (Any, WasmClientState),
            (Any, WasmConsensusState),
            (Any, WasmClientMessage),
        ]:
            WasmEncodingComponents::Provider,

        [
            (Any, WasmTendermintClientState),
            (WasmTendermintClientState, Any),
        ]:
            EncodeWasmTendermintClientState,

        (TendermintConsensusState, Any):
            EncodeViaWasmConsensusState,

        (Any, TendermintConsensusState):
            DecodeViaWasmConsensusState,

    }
}

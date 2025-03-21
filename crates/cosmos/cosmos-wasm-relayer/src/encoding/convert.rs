use cgp::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosClientEncodingComponents;
use hermes_cosmos_chain_components::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_wasm_encoding_components::components::WasmEncodingComponents;
use hermes_wasm_encoding_components::impls::strategies::consensus_state::{
    DecodeViaWasmConsensusState, EncodeViaWasmConsensusState,
};
use hermes_wasm_encoding_components::types::client_message::WasmClientMessage;
use hermes_wasm_encoding_components::types::client_state::WasmClientState;
use hermes_wasm_encoding_components::types::consensus_state::WasmConsensusState;
use ibc::core::commitment_types::merkle::MerkleProof;
use ibc_proto::ibc::core::commitment::v1::MerkleProof as ProtoMerkleProof;
use prost_types::Any;

use crate::types::client_state::{EncodeWasmTendermintClientState, WasmTendermintClientState};

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

use cgp_core::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_cosmos_chain_components::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_protobuf_encoding_components::types::Protobuf;
use hermes_relayer_components::chain::traits::types::proof::ViaCommitmentProof;
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::{ProtoWasmClientState, WasmClientState};
use hermes_wasm_client_components::types::consensus_state::{
    ProtoWasmConsensusState, WasmConsensusState,
};
use ibc::core::commitment_types::merkle::MerkleProof;
use ibc_proto::ibc::core::commitment::v1::MerkleProof as ProtoMerkleProof;
use prost_types::Any;

pub struct WasmCosmosEncoderComponents;

delegate_components! {
    WasmCosmosEncoderComponents {
        [
            (Protobuf, Vec<u8>),
            (ViaCommitmentProof, Vec<u8>),

            (Any, TendermintClientState),
            (Protobuf, TendermintClientState),
            (Protobuf, ProtoTendermintClientState),

            (Any, TendermintConsensusState),
            (Protobuf,TendermintConsensusState),
            (Protobuf, ProtoTendermintConsensusState),

            (ViaCommitmentProof, MerkleProof),
            (Protobuf, MerkleProof),
            (Protobuf, ProtoMerkleProof),

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

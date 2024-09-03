use cgp::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_cosmos_chain_components::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_protobuf_encoding_components::types::{ViaAny, ViaProtobuf};
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
            (ViaProtobuf, Vec<u8>),
            (ViaCommitmentProof, Vec<u8>),

            (ViaAny, TendermintClientState),
            (ViaProtobuf, TendermintClientState),
            (ViaProtobuf, ProtoTendermintClientState),

            (ViaAny, TendermintConsensusState),
            (ViaProtobuf,TendermintConsensusState),
            (ViaProtobuf, ProtoTendermintConsensusState),

            (ViaCommitmentProof, MerkleProof),
            (ViaProtobuf, MerkleProof),
            (ViaProtobuf, ProtoMerkleProof),

            (ViaProtobuf, Any),
        ]:
            CosmosEncodingComponents,
        [
            (ViaAny, WasmClientState),
            (ViaProtobuf, WasmClientState),
            (ViaProtobuf, ProtoWasmClientState),
            (ViaAny, WasmConsensusState),
            (ViaProtobuf, WasmConsensusState),
            (ViaProtobuf, ProtoWasmConsensusState),
        ]:
            WasmEncodingComponents,
    }
}

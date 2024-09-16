use cgp::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosClientEncodingComponents;
use hermes_cosmos_chain_components::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_relayer_components::chain::traits::types::proof::ViaCommitmentProof;
use hermes_wasm_encoding_components::components::WasmEncodingComponents;
use hermes_wasm_encoding_components::types::client_state::WasmClientState;
use hermes_wasm_encoding_components::types::consensus_state::WasmConsensusState;
use ibc::core::client::types::Height;
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

            (ViaProtobuf, Height),
        ]:
            CosmosClientEncodingComponents,
        [
            (ViaAny, WasmClientState),
            (ViaProtobuf, WasmClientState),
            (ViaAny, WasmConsensusState),
            (ViaProtobuf, WasmConsensusState),
        ]:
            WasmEncodingComponents,
    }
}

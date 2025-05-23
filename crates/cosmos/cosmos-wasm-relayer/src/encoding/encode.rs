use hermes_core::relayer_components::chain::traits::ViaCommitmentProof;
use hermes_cosmos_core::chain_components::encoding::CosmosClientEncodingComponents;
use hermes_cosmos_core::chain_components::types::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_cosmos_core::protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_cosmos_core::wasm_encoding_components::components::WasmEncodingComponents;
use hermes_cosmos_core::wasm_encoding_components::types::{
    WasmClientMessage, WasmClientState, WasmConsensusState,
};
use hermes_prelude::*;
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
            CosmosClientEncodingComponents::Provider,
        [
            (ViaAny, WasmClientState),
            (ViaAny, WasmConsensusState),
            (ViaAny, WasmClientMessage),

            (ViaProtobuf, WasmClientState),
            (ViaProtobuf, WasmConsensusState),
            (ViaProtobuf, WasmClientMessage),
        ]:
            WasmEncodingComponents::Provider,
    }
}

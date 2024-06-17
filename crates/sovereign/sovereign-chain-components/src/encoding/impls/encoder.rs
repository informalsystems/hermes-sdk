use cgp_core::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_cosmos_chain_components::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::{Any, Protobuf};
use hermes_relayer_components::chain::traits::types::proof::ViaCommitmentProof;
use hermes_sovereign_rollup_components::impls::borsh_encode::{EncodeWithBorsh, ViaBorsh};
use hermes_sovereign_rollup_components::types::client_state::{
    ProtoSovereignClientState, SovereignClientState,
};
use hermes_sovereign_rollup_components::types::commitment_proof::JellyfishMerkleProof;
use hermes_sovereign_rollup_components::types::consensus_state::{
    ProtoSovereignConsensusState, SovereignConsensusState,
};
use hermes_wasm_client_components::impls::encoding::components::WasmEncodingComponents;
use hermes_wasm_client_components::types::client_state::{
    DecodeViaWasmClientState, ProtoWasmClientState, WasmClientState,
};
use hermes_wasm_client_components::types::consensus_state::{
    EncodeViaWasmConsensusState, ProtoWasmConsensusState, WasmConsensusState,
};

pub struct SovereignEncoderComponents;

delegate_components! {
    SovereignEncoderComponents {
        [
            (Protobuf, Vec<u8>),
            (ViaCommitmentProof, Vec<u8>),

            (Any, TendermintClientState),
            (Protobuf, TendermintClientState),
            (Protobuf, ProtoTendermintClientState),
            (Any, TendermintConsensusState),
            (Protobuf, TendermintConsensusState),
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

        (Any, SovereignClientState):
            EncodeViaAny<Protobuf>,
        (Protobuf, SovereignClientState):
            ConvertAndEncode<ProtoSovereignClientState>,
        (Protobuf, ProtoSovereignClientState):
            EncodeAsProtobuf,

        (Any, SovereignConsensusState):
            EncodeViaAny<Protobuf>,
        (Protobuf, SovereignConsensusState):
            ConvertAndEncode<ProtoSovereignConsensusState>,
        (Protobuf, ProtoSovereignConsensusState):
            EncodeAsProtobuf,

        (WasmClientState, SovereignClientState):
            DecodeViaWasmClientState,

        (WasmConsensusState, SovereignConsensusState):
            EncodeViaWasmConsensusState,

        (ViaBorsh, JellyfishMerkleProof):
            EncodeWithBorsh,
    }
}

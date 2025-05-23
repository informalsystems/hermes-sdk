use hermes_core::encoding_components::impls::{
    ConvertAndEncode, EncodeUsingStrategy, ReturnEncoded,
};
use hermes_core::relayer_components::chain::traits::ViaCommitmentProof;
use hermes_prelude::*;
use hermes_protobuf_encoding_components::impls::{
    EncodeAsProtobuf, EncodeProtoWithMutBuffer, EncodeViaAny,
};
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use ibc::core::client::types::Height;
use ibc::core::commitment_types::merkle::MerkleProof;
use ibc_proto::ibc::core::commitment::v1::MerkleProof as ProtoMerkleProof;
use prost_types::Any;

use crate::types::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};

pub struct CosmosEncoderComponents;

delegate_components! {
    CosmosEncoderComponents {
        (ViaProtobuf, Vec<u8>): ReturnEncoded,
        (ViaCommitmentProof, Vec<u8>): ReturnEncoded,

        (ViaAny, TendermintClientState): EncodeViaAny<ViaProtobuf>,

        (ViaProtobuf, TendermintClientState): ConvertAndEncode<ProtoTendermintClientState>,
        (ViaProtobuf, ProtoTendermintClientState): EncodeAsProtobuf,

        (ViaAny, TendermintConsensusState): EncodeViaAny<ViaProtobuf>,

        (ViaProtobuf,TendermintConsensusState): ConvertAndEncode<ProtoTendermintConsensusState>,
        (ViaProtobuf, ProtoTendermintConsensusState): EncodeAsProtobuf,

        (ViaCommitmentProof, MerkleProof): EncodeUsingStrategy<ViaProtobuf>,
        (ViaProtobuf, MerkleProof): ConvertAndEncode<ProtoMerkleProof>,
        (ViaProtobuf, ProtoMerkleProof): EncodeAsProtobuf,

        [
            (ViaProtobuf, Any),
            (ViaProtobuf, String),
            (ViaProtobuf, Height),
        ]: EncodeProtoWithMutBuffer,
    }
}

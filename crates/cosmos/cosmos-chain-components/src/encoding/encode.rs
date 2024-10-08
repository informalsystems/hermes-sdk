use cgp::prelude::*;
use hermes_encoding_components::impls::encode::convert_and_encode::ConvertAndEncode;
use hermes_encoding_components::impls::encode::return_encoded::ReturnEncoded;
use hermes_encoding_components::impls::encode::use_strategy::EncodeUsingStrategy;
use hermes_protobuf_encoding_components::impls::encode::buffer::EncodeProtoWithMutBuffer;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use hermes_relayer_components::chain::traits::types::proof::ViaCommitmentProof;
use ibc::core::client::types::Height;
use ibc::core::commitment_types::merkle::MerkleProof;
use ibc_proto::ibc::core::commitment::v1::MerkleProof as ProtoMerkleProof;
use prost_types::Any;

use crate::types::tendermint::{
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

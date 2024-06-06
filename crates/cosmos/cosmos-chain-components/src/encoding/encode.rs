use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::Protobuf;
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
        (Any, TendermintClientState): EncodeViaAny<Protobuf>,

        (Protobuf, TendermintClientState): ConvertAndEncode<ProtoTendermintClientState>,
        (Protobuf, ProtoTendermintClientState): EncodeAsProtobuf,

        (Any, TendermintConsensusState): EncodeViaAny<Protobuf>,

        (Protobuf,TendermintConsensusState): ConvertAndEncode<ProtoTendermintConsensusState>,
        (Protobuf, ProtoTendermintConsensusState): EncodeAsProtobuf,

        (Protobuf, MerkleProof): ConvertAndEncode<ProtoMerkleProof>,
        (Protobuf, ProtoMerkleProof): EncodeAsProtobuf,

        (Protobuf, Any): EncodeAsProtobuf,
    }
}

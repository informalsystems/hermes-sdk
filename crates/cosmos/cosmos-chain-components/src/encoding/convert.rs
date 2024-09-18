use cgp::prelude::*;
use hermes_encoding_components::impls::convert::{ConvertFrom, TryConvertFrom};
use hermes_encoding_components::impls::with_context::WithContext;
use hermes_protobuf_encoding_components::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use ibc::core::commitment_types::merkle::MerkleProof;
use ibc_proto::ibc::core::commitment::v1::MerkleProof as ProtoMerkleProof;
use prost_types::Any;

use crate::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};

pub struct CosmosConverterComponents;

delegate_components! {
    CosmosConverterComponents {
        (TendermintClientState, ProtoTendermintClientState): ConvertFrom,
        (ProtoTendermintClientState, TendermintClientState): TryConvertFrom,

        (TendermintConsensusState, ProtoTendermintConsensusState): ConvertFrom,
        (ProtoTendermintConsensusState, TendermintConsensusState): TryConvertFrom,

        (MerkleProof, ProtoMerkleProof): ConvertFrom,
        (ProtoMerkleProof, MerkleProof): TryConvertFrom,

        (TendermintClientState, Any): EncodeAsAnyProtobuf<ViaProtobuf, WithContext>,
        (Any, TendermintClientState): DecodeAsAnyProtobuf<ViaProtobuf, WithContext>,

        (TendermintConsensusState, Any): EncodeAsAnyProtobuf<ViaProtobuf, WithContext>,
        (Any, TendermintConsensusState): DecodeAsAnyProtobuf<ViaProtobuf, WithContext>,
    }
}
